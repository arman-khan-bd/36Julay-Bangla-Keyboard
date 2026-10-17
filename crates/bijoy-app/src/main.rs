//! Bijoy Bangla App: Desktop Control Center with Virtual Keyboard,
//! System-Wide Keyboard Hook, Live Scratchpad, and Converter.

use bijoy_core::{BijoyEngine, BijoyMode, LIGATURES};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const INDEX_HTML: &str = include_str!("../assets/index.html");
const PORT: u16 = 8321;

fn main() {
    println!("============================================================");
    println!("        বিজয় বাংলা কীবোর্ড | Bijoy Bangla Keyboard");
    println!("         Standard BDS 1738:2018 / 1935:2018 / 1520:2018");
    println!("============================================================");

    // Initialize Windows system-wide hook
    #[cfg(target_os = "windows")]
    {
        println!("[Platform: Windows]");
        println!("Starting native low-level keyboard hook (WH_KEYBOARD_LL)...");
        if let Err(e) = bijoy_driver_win::start_hook() {
            eprintln!("Warning: Could not start Windows hook: {}", e);
        } else {
            println!("✓ Windows system-wide hook active!");
            println!("  Hotkeys: Ctrl+Alt+V (Unicode), Ctrl+Alt+B (Classic), Ctrl+Alt+E (English), F12 (Toggle)");
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("[Platform: Linux]");
        println!("Bijoy Bangla Linux driver loaded.");
        println!("To enable system-wide IBus input source, run: bijoy-cli linux install-ibus");
    }

    let engine = Arc::new(Mutex::new(BijoyEngine::new()));

    // Launch HTTP Server on 127.0.0.1:8321
    let addr = format!("127.0.0.1:{}", PORT);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
            eprintln!("Is another instance of Bijoy already running?");
            return;
        }
    };

    println!("\n✓ Bijoy Control Center UI running at: http://{}", addr);
    println!("Opening web browser...");

    open_browser(&format!("http://{}", addr));

    println!("Press Ctrl+C in terminal to exit.\n");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let engine_clone = Arc::clone(&engine);
            thread::spawn(move || {
                handle_http_client(stream, engine_clone);
            });
        }
    }
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
}

fn handle_http_client(mut stream: TcpStream, engine: Arc<Mutex<BijoyEngine>>) {
    let mut raw_bytes = Vec::new();
    let mut chunk = [0u8; 4096];
    let mut header_end = None;
    let mut content_length = 0usize;

    loop {
        let n = match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(_) => break,
        };
        raw_bytes.extend_from_slice(&chunk[..n]);

        if header_end.is_none() {
            if let Some(pos) = raw_bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                header_end = Some(pos + 4);
                let header_str = String::from_utf8_lossy(&raw_bytes[..pos]);
                for line in header_str.lines() {
                    if line.to_ascii_lowercase().starts_with("content-length:") {
                        if let Ok(l) = line[15..].trim().parse::<usize>() {
                            content_length = l;
                        }
                    }
                }
            }
        }

        if let Some(h_end) = header_end {
            if raw_bytes.len() >= h_end + content_length {
                break;
            }
        }
    }

    let h_end = match header_end {
        Some(pos) => pos,
        None => return,
    };

    let header_str = String::from_utf8_lossy(&raw_bytes[..h_end]);
    let mut lines = header_str.lines();
    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];
    let body_bytes_end = (h_end + content_length).min(raw_bytes.len());
    let body_str = String::from_utf8_lossy(&raw_bytes[h_end..body_bytes_end]);
    let body = &body_str;

    match (method, path) {
        ("GET", "/") => {
            respond_html(&mut stream, INDEX_HTML);
        }
        ("GET", "/api/status") => {
            #[cfg(target_os = "windows")]
            let mode = bijoy_driver_win::get_current_mode();
            #[cfg(not(target_os = "windows"))]
            let mode = engine.lock().unwrap().mode;

            let mode_str = match mode {
                BijoyMode::Unicode => "Unicode",
                BijoyMode::Classic => "Classic",
                BijoyMode::Avro => "Avro",
                BijoyMode::English => "English",
            };
            let json = format!(
                r#"{{"mode":"{}","platform":"{}","hook_active":true}}"#,
                mode_str,
                std::env::consts::OS
            );
            respond_json(&mut stream, &json);
        }
        ("POST", "/api/mode") => {
            let mode = if body.contains("Classic") {
                BijoyMode::Classic
            } else if body.contains("Avro") {
                BijoyMode::Avro
            } else if body.contains("English") {
                BijoyMode::English
            } else {
                BijoyMode::Unicode
            };

            #[cfg(target_os = "windows")]
            bijoy_driver_win::set_current_mode(mode);

            engine.lock().unwrap().set_mode(mode);
            respond_json(&mut stream, r#"{"ok":true}"#);
        }
        ("POST", "/api/reset") => {
            engine.lock().unwrap().reset();
            respond_json(&mut stream, r#"{"ok":true}"#);
        }
        ("POST", "/api/type") => {
            let ch = extract_char_from_json(body);
            let mut eng = engine.lock().unwrap();
            let out = eng.process_char(ch);
            let escaped_text = escape_json_str(&out.insert_text);
            let json = format!(
                r#"{{"consumed":{},"delete_count":{},"insert_text":"{}"}}"#,
                out.consumed, out.delete_count, escaped_text
            );
            respond_json(&mut stream, &json);
        }
        ("POST", "/api/convert") => {
            let (text, direction) = extract_convert_args(body);
            let converted = if direction == "to_classic" || direction == "avro_to_36july" {
                bijoy_core::avro_to_36july(&text)
            } else if direction == "phonetic" {
                bijoy_core::avro_phonetic_to_unicode(&text)
            } else {
                bijoy_core::bijoy_36july_to_avro(&text)
            };
            let escaped = escape_json_str(&converted);
            let json = format!(r#"{{"result":"{}"}}"#, escaped);
            respond_json(&mut stream, &json);
        }
        ("POST", "/api/suggestions") => {
            let (query, _) = extract_convert_args(body);
            let suggestions = bijoy_core::get_word_suggestions(&query, 6);
            let mut json = String::from("[");
            for (idx, word) in suggestions.iter().enumerate() {
                if idx > 0 {
                    json.push(',');
                }
                json.push_str(&format!(r#""{}""#, escape_json_str(word)));
            }
            json.push(']');
            respond_json(&mut stream, &json);
        }
        ("GET", "/api/ligatures") => {
            let mut json = String::from("[");
            for (idx, lig) in LIGATURES.iter().enumerate() {
                if idx > 0 {
                    json.push(',');
                }
                json.push_str(&format!(
                    r#"{{"name":"{}","unicode":"{}","keys":"{}","sutonny":"{}"}}"#,
                    lig.name,
                    escape_json_str(lig.unicode),
                    lig.keys,
                    escape_json_str(lig.sutonny)
                ));
            }
            json.push(']');
            respond_json(&mut stream, &json);
        }
        _ => {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        }
    }
}

fn respond_html(stream: &mut TcpStream, html: &str) {
    let bytes = html.as_bytes();
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.write_all(bytes);
}

fn respond_json(stream: &mut TcpStream, json: &str) {
    let bytes = json.as_bytes();
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.write_all(bytes);
}

fn escape_json_str(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

fn extract_char_from_json(body: &str) -> char {
    if let Some(pos) = body.find("\"char\"") {
        let after_key = &body[pos + 6..];
        let after_colon = after_key.trim_start().strip_prefix(':').unwrap_or(after_key).trim_start();
        if let Some(val_str) = after_colon.strip_prefix('"') {
            if val_str.starts_with("\\x08") || val_str.starts_with("\\b") {
                return '\x08';
            }
            let mut chars = val_str.chars();
            if let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(c2) = chars.next() {
                        return match c2 {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'b' => '\x08',
                            _ => c2,
                        };
                    }
                }
                return c;
            }
        }
    }
    '\0'
}

fn extract_convert_args(body: &str) -> (String, String) {
    let mut text = String::new();
    let mut direction = String::from("to_unicode");

    if let Some(pos) = body.find("\"direction\"") {
        let after_key = &body[pos + 11..];
        let after_colon = after_key.trim_start().strip_prefix(':').unwrap_or(after_key).trim_start();
        if let Some(val) = after_colon.strip_prefix('"') {
            if let Some(end) = val.find('"') {
                direction = val[..end].to_string();
            }
        }
    }

    if let Some(pos) = body.find("\"text\"") {
        let after_key = &body[pos + 6..];
        let after_colon = after_key.trim_start().strip_prefix(':').unwrap_or(after_key).trim_start();
        if let Some(val) = after_colon.strip_prefix('"') {
            let mut chars = val.chars();
            while let Some(c) = chars.next() {
                if c == '"' {
                    break;
                }
                if c == '\\' {
                    if let Some(c2) = chars.next() {
                        match c2 {
                            '"' => text.push('"'),
                            '\\' => text.push('\\'),
                            'n' => text.push('\n'),
                            'r' => text.push('\r'),
                            't' => text.push('\t'),
                            other => text.push(other),
                        }
                    }
                } else {
                    text.push(c);
                }
            }
        }
    }

    (text, direction)
}
