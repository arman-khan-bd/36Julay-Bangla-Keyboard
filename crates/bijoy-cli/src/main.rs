//! Bijoy Bangla CLI: Interactive REPL, File Converter, Keyboard Layout viewer,
//! and Background Typing Daemon.

use bijoy_core::{classic_to_unicode, unicode_to_classic, BijoyEngine, BijoyMode, LIGATURES};
use std::env;
use std::fs;
use std::io::{self, Write};

fn print_help() {
    println!(
r#"Bijoy Bangla Keyboard (BDS Standard BDS 1738:2018)
High-performance Rust implementation for Windows + Linux

USAGE:
    bijoy-cli <COMMAND> [OPTIONS]

COMMANDS:
    hook, daemon        Run background system-wide keyboard hook (Windows / Linux)
    repl                Start interactive terminal typing REPL
    convert             Convert text or files between Bijoy Classic (SutonnyMJ) and Unicode
    layout              Show Bijoy keyboard layout map and ligature cheat sheet
    linux               Linux IBus installation and distro configuration instructions
    help                Print this help message

EXAMPLES:
    bijoy-cli hook
    bijoy-cli repl
    bijoy-cli convert "wWwRUvj evsjv‡`k" --to-unicode
    bijoy-cli convert "আমার সোনার বাংলা" --to-classic
    bijoy-cli convert -i old_doc.txt -o new_doc.txt --to-unicode
    bijoy-cli layout
    bijoy-cli linux install-ibus
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "hook" | "daemon" => run_hook_daemon(),
        "repl" => run_repl(),
        "convert" => run_convert(&args[2..]),
        "layout" => print_layout(),
        "linux" => run_linux_commands(&args[2..]),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Unknown command: {}", other);
            print_help();
        }
    }
}

fn run_hook_daemon() {
    println!("============================================================");
    println!("  Bijoy Bangla System-Wide Keyboard Hook (Rust)");
    println!("  Standard BDS 1738:2018 / 1935:2018 / 1520:2018");
    println!("============================================================");

    #[cfg(target_os = "windows")]
    {
        use bijoy_driver_win::start_hook;

        println!("Target OS: Windows (Native WH_KEYBOARD_LL + SendInput)");
        println!("Hotkeys:");
        println!("  Ctrl + Alt + V  => Bijoy UNICODE Mode (Default, modern apps & web)");
        println!("  Ctrl + Alt + B  => Bijoy CLASSIC Mode (SutonnyMJ ANSI fonts)");
        println!("  Ctrl + Alt + E  => ENGLISH Mode (Normal typing pass-through)");
        println!("  F12             => Toggle English <-> Last Bijoy Mode\n");

        if let Err(e) = start_hook() {
            eprintln!("Error starting Windows hook: {}", e);
            return;
        }

        println!("Hook is actively running! Type into Notepad, Word, Chrome, etc.");
        println!("Press Ctrl+C to terminate this daemon.");

        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("Target OS: Linux (All Distros)");
        println!("{}", bijoy_driver_linux::linux_distro_instructions());
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        println!("Background hook is supported on Windows and Linux.");
    }
}

fn run_repl() {
    println!("=== Bijoy Bangla Interactive REPL ===");
    println!("Type English keys according to Bijoy layout to see Bengali output!");
    println!("Commands: /unicode, /classic, /english, /reset, /quit\n");

    let mut engine = BijoyEngine::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        let mode_str = match engine.mode {
            BijoyMode::Unicode => "Unicode",
            BijoyMode::Classic => "Classic",
            BijoyMode::Avro => "Avro",
            BijoyMode::English => "English",
        };
        print!("bijoy[{}]> ", mode_str);
        io::stdout().flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).is_err() {
            break;
        }
        let line = input.trim();

        if line == "/quit" || line == "/exit" {
            break;
        } else if line == "/unicode" {
            engine.set_mode(BijoyMode::Unicode);
            println!("Switched to Unicode mode.");
            continue;
        } else if line == "/classic" {
            engine.set_mode(BijoyMode::Classic);
            println!("Switched to Classic (SutonnyMJ) mode.");
            continue;
        } else if line == "/avro" {
            engine.set_mode(BijoyMode::Avro);
            println!("Switched to Avro Phonetic mode.");
            continue;
        } else if line == "/english" {
            engine.set_mode(BijoyMode::English);
            println!("Switched to English mode.");
            continue;
        } else if line == "/reset" {
            engine.reset();
            println!("Engine reset.");
            continue;
        }

        let mut output = String::new();
        for ch in line.chars() {
            let out = engine.process_char(ch);
            if out.delete_count > 0 {
                for _ in 0..out.delete_count {
                    output.pop();
                }
            }
            output.push_str(&out.insert_text);
        }
        engine.reset();
        println!("=> {}", output);
    }
}

fn run_convert(args: &[String]) {
    let mut to_unicode = true;
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut inline_text: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--to-unicode" | "-u" => to_unicode = true,
            "--to-classic" | "-c" => to_unicode = false,
            "-i" | "--input" => {
                if i + 1 < args.len() {
                    input_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            text if !text.starts_with('-') => {
                inline_text = Some(text.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    if let Some(path) = input_file {
        match fs::read_to_string(&path) {
            Ok(content) => {
                let converted = if to_unicode {
                    classic_to_unicode(&content)
                } else {
                    unicode_to_classic(&content)
                };
                if let Some(out_path) = output_file {
                    if let Err(e) = fs::write(&out_path, &converted) {
                        eprintln!("Failed to write to {}: {}", out_path, e);
                    } else {
                        println!("Converted file saved to: {}", out_path);
                    }
                } else {
                    println!("{}", converted);
                }
            }
            Err(e) => eprintln!("Error reading file {}: {}", path, e),
        }
    } else if let Some(text) = inline_text {
        let converted = if to_unicode {
            classic_to_unicode(&text)
        } else {
            unicode_to_classic(&text)
        };
        println!("{}", converted);
    } else {
        eprintln!("Please specify text or an input file (-i <file>).");
    }
}

fn print_layout() {
    print!(
        "{}",
r#"========================================================================
           BIJOY BANGLA KEYBOARD LAYOUT (BDS 1738:2018)
========================================================================
[Number Row]
  Normal:   `  1  2  3  4  5  6  7  8  9  0  -  =
  Bangla:   ‘  ১  ২  ৩  ৪  ৫  ৬  ৭  ৮  ৯  ০  -  =
  Shift:    “  !  @  #  ৳  %  ^  ঁ  *  (  )  _  +

[QWERTY Row]
  Normal:   q  w  e  r  t  y  u  i  o  p  [  ]  \
  Bangla:   ঙ  য  ড  প  ট  চ  জ  হ  গ  ড়  [  ]  ৎ
  Shift:    ং  য়  ঢ  ফ  ঠ  ছ  ঝ  ঞ  ঘ  ঢ়  {  }  ঃ

[ASDF Row]
  Normal:   a  s  d  f  g  h  j  k  l  ;  '
  Bangla:   ৃ  ু  ি  া  ্  ব  ক  ত  দ  ;  '
  Shift:    র্  ূ  ী  অ  ।  ভ  খ  থ  ধ  :  "

[ZXCV Row]
  Normal:   z  x  c  v  b  n  m  ,  .  /
  Bangla:   ্র  ও  ে  র  ন  স  ম  ,  .  /
  Shift:    ্য  ৌ  ৈ  ল  ণ  ষ  শ  <  >  ?

[Independent Vowels (with Link 'g')]
  F: অ    gf: আ    gd: ই    gD: ঈ    gs: উ    gS: ঊ
  ga: ঋ   gc: এ    gC: ঐ    x: ও     gX: ঔ

[Special Combinations]
  A + consonant: র্ক (Ref)     z: ্র (r-phala)    Z: ্য (ya-phala)
  c + cons + f:  কো (O-kar)   c + cons + X:  কৌ (Ou-kar)
  g + G: ॥ (Double Dari)       Shift+4: ৳ (Bangla Taka)

[Common Ligatures]
  jgk: ক্ত    jgN: ক্ষ    ugI: জ্ঞ    Igy: ঞ্চ    igm: হ্ম
  kgk: ত্ত    lgl: দ্দ    lgL: দ্ধ    bgk: ন্ত    bgl: ন্দ
========================================================================
"#
    );

    println!("Total ligatures defined in PDF database: {}", LIGATURES.len());
}

fn run_linux_commands(args: &[String]) {
    if args.is_empty() {
        println!("{}", bijoy_driver_linux::linux_distro_instructions());
        return;
    }

    match args[0].as_str() {
        "install-ibus" => {
            let binary = env::current_exe()
                .unwrap_or_else(|_| std::path::PathBuf::from("bijoy-cli"))
                .to_string_lossy()
                .to_string();
            match bijoy_driver_linux::install_ibus_component(false, &binary) {
                Ok(path) => println!("IBus component installed successfully at: {}", path.display()),
                Err(e) => eprintln!("Failed to install IBus component: {}", e),
            }
        }
        "instructions" | "distros" => {
            println!("{}", bijoy_driver_linux::linux_distro_instructions());
        }
        _ => {
            println!("{}", bijoy_driver_linux::linux_distro_instructions());
        }
    }
}
