//! Stateful typing engine for Bijoy Bangla Keyboard.
//! Supports Unicode mode, Classic (SutonnyMJ) mode, and English pass-through.

use crate::layout::{map_ancient_or_special, map_key, map_linked_vowel, KeyType};
use crate::ligatures::LIGATURES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BijoyMode {
    Unicode, // Ctrl+Alt+V (বিজয় ৩৬জুলাই)
    Classic, // Ctrl+Alt+B (বিজয় ক্লাসিক SutonnyMJ)
    Avro,    // F12 (অভ্র ফোনেটিক)
    English, // Ctrl+Alt+E (English)
}

impl Default for BijoyMode {
    fn default() -> Self {
        BijoyMode::Unicode
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineOutput {
    /// Number of characters to backspace in the target app before inserting
    pub delete_count: usize,
    /// String to insert into the target app
    pub insert_text: String,
    /// Whether this key was consumed by the IME
    pub consumed: bool,
}

impl EngineOutput {
    pub fn pass_through() -> Self {
        Self {
            delete_count: 0,
            insert_text: String::new(),
            consumed: false,
        }
    }

    pub fn insert(text: impl Into<String>) -> Self {
        Self {
            delete_count: 0,
            insert_text: text.into(),
            consumed: true,
        }
    }

    pub fn replace(delete_count: usize, text: impl Into<String>) -> Self {
        Self {
            delete_count,
            insert_text: text.into(),
            consumed: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BijoyEngine {
    pub mode: BijoyMode,
    pending_link: bool,
    pending_ref: bool,
    pending_pre_kar: Option<char>, // 'ে', 'ি', 'ৈ'
    last_consonant_cluster: String,
    consonant_with_ekar: bool,
    consonant_just_typed: bool,
    last_sutonny_key: Option<char>,
    avro_buffer: String,
}

impl Default for BijoyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BijoyEngine {
    pub fn new() -> Self {
        Self {
            mode: BijoyMode::Unicode,
            pending_link: false,
            pending_ref: false,
            pending_pre_kar: None,
            last_consonant_cluster: String::new(),
            consonant_with_ekar: false,
            consonant_just_typed: false,
            last_sutonny_key: None,
            avro_buffer: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pending_link = false;
        self.pending_ref = false;
        self.pending_pre_kar = None;
        self.last_consonant_cluster.clear();
        self.consonant_with_ekar = false;
        self.consonant_just_typed = false;
        self.last_sutonny_key = None;
        self.avro_buffer.clear();
    }

    pub fn set_mode(&mut self, mode: BijoyMode) {
        self.mode = mode;
        self.reset();
    }

    /// Retrieve live query string and candidate word suggestions for preview window
    pub fn get_preview_info(&self) -> (String, Vec<String>) {
        if self.mode == BijoyMode::Avro {
            if !self.avro_buffer.is_empty() {
                let sugs = crate::dictionary::get_word_suggestions(&self.avro_buffer, 5);
                return (self.avro_buffer.clone(), sugs);
            }
        } else if self.mode == BijoyMode::Unicode {
            if let Some(pre) = self.pending_pre_kar {
                return (format!("+{}", pre), vec![format!("{}...", pre)]);
            } else if self.pending_link {
                return ("্ (Link)".to_string(), vec!["যুক্তাক্ষর".to_string()]);
            }
        }
        (String::new(), Vec::new())
    }

    /// Process a printable character key event.
    pub fn process_char(&mut self, ch: char) -> EngineOutput {
        if self.mode == BijoyMode::English {
            return EngineOutput::pass_through();
        }

        // Backspace handling
        if ch == '\x08' {
            if self.mode == BijoyMode::Avro && !self.avro_buffer.is_empty() {
                self.avro_buffer.pop();
                return EngineOutput::pass_through();
            }
            self.reset();
            return EngineOutput::pass_through();
        }

        match self.mode {
            BijoyMode::Unicode => self.process_unicode(ch),
            BijoyMode::Classic => self.process_classic(ch),
            BijoyMode::Avro => self.process_avro(ch),
            BijoyMode::English => EngineOutput::pass_through(),
        }
    }

    /// Avro phonetic real-time processing
    fn process_avro(&mut self, ch: char) -> EngineOutput {
        if ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t' {
            if !self.avro_buffer.is_empty() {
                let bangla = crate::avro::avro_phonetic_to_unicode(&self.avro_buffer);
                let del = self.avro_buffer.chars().count();
                self.avro_buffer.clear();
                return EngineOutput::replace(del, format!("{}{}", bangla, ch));
            }
            return EngineOutput::pass_through();
        }

        if ch.is_ascii_punctuation() {
            if !self.avro_buffer.is_empty() {
                let bangla = crate::avro::avro_phonetic_to_unicode(&self.avro_buffer);
                let del = self.avro_buffer.chars().count();
                self.avro_buffer.clear();
                return EngineOutput::replace(del, format!("{}{}", bangla, ch));
            }
            return EngineOutput::pass_through();
        }

        if ch.is_ascii_alphabetic() {
            self.avro_buffer.push(ch);
            return EngineOutput::pass_through();
        }

        if !self.avro_buffer.is_empty() {
            let bangla = crate::avro::avro_phonetic_to_unicode(&self.avro_buffer);
            let del = self.avro_buffer.chars().count();
            self.avro_buffer.clear();
            return EngineOutput::replace(del, format!("{}{}", bangla, ch));
        }

        EngineOutput::pass_through()
    }

    /// Internal unicode processing logic for Bijoy 36July (BDS 1738)
    fn process_unicode(&mut self, ch: char) -> EngineOutput {
        // Check for 'g' (Link / Virama)
        if ch == 'g' {
            if self.pending_link {
                // Double 'g' emits standalone Virama / Hasanta
                self.pending_link = false;
                self.consonant_with_ekar = false;
                self.consonant_just_typed = false;
                return EngineOutput::insert("্");
            } else {
                self.pending_link = true;
                self.consonant_just_typed = false;
                return EngineOutput::insert(""); // buffered silently
            }
        }

        // If link 'g' was pressed previously
        if self.pending_link {
            self.pending_link = false;

            // Check linked independent vowel (e.g., g + f => আ, g + d => ই)
            if let Some(vowel) = map_linked_vowel(ch) {
                self.reset();
                return EngineOutput::insert(vowel.to_string());
            }

            // Check ancient/special/Assamese/Vedic characters (e.g. g + 1..8, g + ,, g + .)
            if let Some(special) = map_ancient_or_special(ch) {
                self.reset();
                return EngineOutput::insert(special.to_string());
            }

            // If next key is a consonant, it's a ligature (consonant + virama + consonant)
            if let Some(KeyType::Consonant(c2)) = map_key(ch) {
                if self.consonant_with_ekar {
                    // cluster ended with 'ে' (e.g., 'কে' from 'c' + 'j')
                    // Now user pressed 'g' + 'k' ('ত'):
                    // Replace 'ে' with '্' + c2 + 'ে' => 'ক্তে'
                    let mut s = String::new();
                    s.push('\u{09CD}');
                    s.push(c2);
                    s.push('ে');
                    self.last_consonant_cluster.push('\u{09CD}');
                    self.last_consonant_cluster.push(c2);
                    self.consonant_just_typed = true;
                    return EngineOutput::replace(1, s);
                } else if !self.last_consonant_cluster.is_empty() {
                    let mut s = String::new();
                    s.push('\u{09CD}'); // Virama
                    s.push(c2);
                    self.last_consonant_cluster.push_str(&s);
                    self.consonant_just_typed = true;
                    return EngineOutput::insert(s);
                } else {
                    self.last_consonant_cluster = c2.to_string();
                    self.consonant_just_typed = true;
                    return EngineOutput::insert(format!("্{}", c2));
                }
            }

            // If not a vowel or consonant, output virama then process ch
            let mut s = String::from("্");
            if let Some(kt) = map_key(ch) {
                match kt {
                    KeyType::Consonant(c) | KeyType::Vowel(c) | KeyType::VowelSign(c)
                    | KeyType::Modifier(c) | KeyType::Punctuation(c) | KeyType::Digit(c) => {
                        s.push(c);
                        self.reset();
                        return EngineOutput::insert(s);
                    }
                    _ => {}
                }
            }
            self.reset();
            return EngineOutput::pass_through();
        }

        // Check for 'A' (Ref: র্)
        if ch == 'A' {
            self.pending_ref = true;
            self.consonant_just_typed = false;
            return EngineOutput::insert(""); // buffered silently
        }

        // Check for 'z' (r-phala: ্র)
        if ch == 'z' {
            if self.consonant_with_ekar {
                // e.g. 'কে' + 'z' => 'ক্রে'
                self.consonant_just_typed = true;
                return EngineOutput::replace(1, "্রে");
            } else if !self.last_consonant_cluster.is_empty() {
                let r_phala = "্\u{09B0}"; // virama + ra
                self.last_consonant_cluster.push_str(r_phala);
                self.consonant_just_typed = true;
                return EngineOutput::insert(r_phala);
            } else {
                return EngineOutput::insert("্র");
            }
        }

        // Check for 'Z' (ya-phala: ্য)
        if ch == 'Z' {
            if self.consonant_with_ekar {
                // e.g. 'কে' + 'Z' => 'ক্যে'
                self.consonant_just_typed = true;
                return EngineOutput::replace(1, "্যে");
            } else if !self.last_consonant_cluster.is_empty() {
                let ya_phala = "্\u{09AF}"; // virama + ya
                self.last_consonant_cluster.push_str(ya_phala);
                self.consonant_just_typed = true;
                return EngineOutput::insert(ya_phala);
            } else {
                return EngineOutput::insert("্য");
            }
        }

        // Check for O-kar ('ো') and Ou-kar ('ৌ') right after an e-kar:
        // 'c' + consonant + 'f' => 'ো', 'c' + consonant + 'X' => 'ৌ'
        if self.consonant_with_ekar {
            if ch == 'f' {
                self.consonant_with_ekar = false;
                self.consonant_just_typed = false;
                // Delete 'ে', replace with 'ো'
                return EngineOutput::replace(1, "ো");
            } else if ch == 'X' {
                self.consonant_with_ekar = false;
                self.consonant_just_typed = false;
                // Delete 'ে', replace with 'ৌ'
                return EngineOutput::replace(1, "ৌ");
            }
            self.consonant_with_ekar = false;
        }

        // Check if pre-kar key pressed (c => ে, d => ি, C => ৈ)
        if ch == 'c' || ch == 'd' || ch == 'C' {
            let kar = match ch {
                'c' => 'ে',
                'd' => 'ি',
                'C' => 'ৈ',
                _ => unreachable!(),
            };

            // Traditional Bijoy typing order: pre-kar typed BEFORE consonant (e.g. ি + ক => কি)
            // Buffer pre-kar in memory; DO NOT emit unattached combining mark to target application!
            // When the consonant is typed, consonant + pre-kar are inserted together atomically with 0 backspaces!
            self.pending_pre_kar = Some(kar);
            self.consonant_just_typed = false;
            self.consonant_with_ekar = false;
            self.last_consonant_cluster.clear();
            return EngineOutput::insert(""); // buffered silently
        }

        // Key lookup
        if let Some(key_type) = map_key(ch) {
            match key_type {
                KeyType::Consonant(c) => {
                    let mut s = String::new();

                    if self.pending_ref {
                        s.push('র');
                        s.push('\u{09CD}');
                        self.pending_ref = false;
                    }

                    s.push(c);
                    self.last_consonant_cluster = s.clone();
                    self.consonant_just_typed = true;

                    if let Some(pre_kar) = self.pending_pre_kar.take() {
                        // In traditional Bijoy: pre-kar was typed before this consonant.
                        // In Unicode logical order: Consonant + PreKar!
                        // Both are emitted together in a single atomic insertion!
                        s.push(pre_kar);
                        self.consonant_with_ekar = pre_kar == 'ে';
                    } else {
                        self.consonant_with_ekar = false;
                    }

                    EngineOutput::insert(s)
                }
                KeyType::Vowel(v) => {
                    let mut s = String::new();
                    if let Some(pre) = self.pending_pre_kar.take() {
                        s.push(pre);
                    }
                    s.push(v);
                    self.reset();
                    EngineOutput::insert(s)
                }
                KeyType::VowelSign(vs) => {
                    self.consonant_just_typed = false;
                    self.consonant_with_ekar = false;
                    EngineOutput::insert(vs.to_string())
                }
                KeyType::Modifier(m) => {
                    self.consonant_just_typed = false;
                    self.consonant_with_ekar = false;
                    EngineOutput::insert(m.to_string())
                }
                KeyType::Punctuation(p) => {
                    self.reset();
                    EngineOutput::insert(p.to_string())
                }
                KeyType::Digit(d) => {
                    self.reset();
                    EngineOutput::insert(d.to_string())
                }
                _ => {
                    self.reset();
                    EngineOutput::pass_through()
                }
            }
        } else {
            // Unmapped key (space, enter, other symbols)
            if let Some(pre) = self.pending_pre_kar.take() {
                // If pre-kar was typed followed by space/other, flush pre-kar first
                self.reset();
                EngineOutput::insert(format!("{}{}", pre, ch))
            } else {
                self.reset();
                EngineOutput::pass_through()
            }
        }
    }

    /// Internal classic / SutonnyMJ processing logic
    fn process_classic(&mut self, ch: char) -> EngineOutput {
        if ch == 'g' {
            if self.pending_link {
                self.pending_link = false;
                return EngineOutput::insert("&");
            } else {
                self.pending_link = true;
                return EngineOutput::insert("");
            }
        }

        if self.pending_link {
            self.pending_link = false;
            // Independent vowels
            let vowel = match ch {
                'f' => Some("Av"),
                'd' => Some("B"),
                'D' => Some("C"),
                's' => Some("D"),
                'S' => Some("E"),
                'a' => Some("F"),
                'c' => Some("G"),
                'C' => Some("H"),
                'X' => Some("J"),
                _ => None,
            };
            if let Some(v) = vowel {
                return EngineOutput::insert(v);
            }

            // Check if key combination forms a known SutonnyMJ ligature
            if let Some(prev) = self.last_sutonny_key {
                let seq = format!("{}g{}", prev, ch);
                for lig in LIGATURES {
                    if lig.keys == seq {
                        // delete previous char, insert ligature
                        self.last_sutonny_key = None;
                        return EngineOutput::replace(1, lig.sutonny);
                    }
                }
            }

            // fallback virama + char
            let mut res = String::from("&");
            if let Some(s) = map_sutonny_char(ch) {
                res.push_str(s);
            }
            return EngineOutput::insert(res);
        }

        if let Some(s) = map_sutonny_char(ch) {
            self.last_sutonny_key = Some(ch);
            EngineOutput::insert(s)
        } else {
            self.last_sutonny_key = None;
            EngineOutput::pass_through()
        }
    }
}

/// Direct SutonnyMJ glyph mapping for classic mode
pub fn map_sutonny_char(ch: char) -> Option<&'static str> {
    match ch {
        'j' => Some("K"),
        'J' => Some("L"),
        'o' => Some("M"),
        'O' => Some("N"),
        'q' => Some("O"),
        'y' => Some("P"),
        'Y' => Some("Q"),
        'u' => Some("R"),
        'U' => Some("S"),
        'I' => Some("T"),
        't' => Some("U"),
        'T' => Some("V"),
        'e' => Some("W"),
        'E' => Some("X"),
        'B' => Some("Y"),
        'k' => Some("Z"),
        'K' => Some("_"),
        'l' => Some("`"),
        'L' => Some("a"),
        'b' => Some("b"),
        'r' => Some("c"),
        'R' => Some("d"),
        'h' => Some("e"),
        'H' => Some("f"),
        'm' => Some("g"),
        'w' => Some("h"),
        'v' => Some("i"),
        'V' => Some("j"),
        'M' => Some("k"),
        'N' => Some("l"),
        'n' => Some("m"),
        'i' => Some("n"),
        'p' => Some("o"),
        'P' => Some("p"),
        'W' => Some("q"),
        '\\' => Some("r"),
        'Q' => Some("s"),
        '|' => Some("t"),
        '&' => Some("u"),
        'f' => Some("v"),
        'd' => Some("w"),
        'D' => Some("x"),
        's' => Some("y"),
        'S' => Some("~"),
        'a' => Some("„"),
        'c' => Some("†"),
        'C' => Some("ˆ"),
        'X' => Some("Š"),
        'x' => Some("I"),
        'F' => Some("A"),
        'A' => Some("©"),
        'z' => Some("ª"),
        'Z' => Some("¨"),
        'G' => Some("|"),
        '$' => Some("$"),
        '0' => Some("0"),
        '1' => Some("1"),
        '2' => Some("2"),
        '3' => Some("3"),
        '4' => Some("4"),
        '5' => Some("5"),
        '6' => Some("6"),
        '7' => Some("7"),
        '8' => Some("8"),
        '9' => Some("9"),
        _ => None,
    }
}
