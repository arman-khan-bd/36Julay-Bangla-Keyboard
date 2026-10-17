//! Avro Phonetic Engine and Avro ⇄ 36July (Bijoy) Bidirectional Converter.
//! Translates between Avro Phonetic, Bengali Unicode, and 36July (Bijoy Classic / SutonnyMJ).

use crate::converter::{classic_to_unicode, unicode_to_classic};
use crate::dictionary::get_word_suggestions;

/// Convert Avro Phonetic English Romanized text into standard Bengali Unicode.
/// Example: "ami banglay gan gai" => "আমি বাংলায় গান গাই"
pub fn avro_phonetic_to_unicode(input: &str) -> String {
    let mut result = String::new();
    let words: Vec<&str> = input.split_whitespace().collect();

    for (w_idx, word) in words.iter().enumerate() {
        if w_idx > 0 {
            result.push(' ');
        }
        let dict = get_word_suggestions(word, 1);
        if let Some(first) = dict.into_iter().next() {
            result.push_str(&first);
        } else {
            result.push_str(&transliterate_avro_word(word));
        }
    }

    result
}

fn transliterate_avro_word(w: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = w.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Multi-char matches
        if i + 3 <= chars.len() {
            let three: String = chars[i..i + 3].iter().collect();
            if three == "rri" {
                if out.is_empty() || !is_consonant(chars[i - 1]) {
                    out.push('ঋ');
                } else {
                    out.push('ৃ');
                }
                i += 3;
                continue;
            }
        }

        if i + 2 <= chars.len() {
            let two: String = chars[i..i + 2].iter().collect();
            let matched = match two.as_str() {
                "kh" => Some("খ"),
                "gh" => Some("ঘ"),
                "Ng" | "ng" => Some("ঙ"),
                "ch" => Some("ছ"),
                "jh" => Some("ঝ"),
                "Th" => Some("ঠ"),
                "Dh" => Some("ঢ"),
                "th" => Some("থ"),
                "dh" => Some("ধ"),
                "ph" => Some("ফ"),
                "bh" => Some("ভ"),
                "sh" => Some("শ"),
                "Sh" => Some("ষ"),
                "Rh" => Some("ঢ়"),
                "oi" => Some("ঐ"),
                "ou" => Some("ঔ"),
                "aa" => Some("আ"),
                "ee" => Some("ঈ"),
                "oo" => Some("ঊ"),
                _ => None,
            };

            if let Some(replacement) = matched {
                out.push_str(replacement);
                i += 2;
                continue;
            }
        }

        // Single char matches
        let ch = chars[i];
        let single = match ch {
            'k' => "ক",
            'g' => "গ",
            'c' => "চ",
            'j' => "জ",
            'T' => "ট",
            'D' => "ড",
            'N' => "ণ",
            't' => "ত",
            'd' => "দ",
            'n' => "ন",
            'p' => "প",
            'f' => "ফ",
            'b' => "ব",
            'v' => "ভ",
            'm' => "ম",
            'z' => "য",
            'r' => "র",
            'l' => "ল",
            'S' => "ষ",
            's' => "স",
            'h' => "হ",
            'R' => "ড়",
            'y' => "য়",
            'a' => if out.is_empty() { "অ" } else { "া" },
            'A' => if out.is_empty() { "আ" } else { "া" },
            'i' => if out.is_empty() { "ই" } else { "ি" },
            'I' => if out.is_empty() { "ঈ" } else { "ী" },
            'u' => if out.is_empty() { "উ" } else { "ু" },
            'U' => if out.is_empty() { "ঊ" } else { "ূ" },
            'e' => if out.is_empty() { "এ" } else { "ে" },
            'o' => if out.is_empty() { "ও" } else { "ো" },
            _ => "",
        };

        if !single.is_empty() {
            out.push_str(single);
        } else {
            out.push(ch);
        }
        i += 1;
    }

    out
}

fn is_consonant(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'b'..='d' | 'f'..='h' | 'j'..='n' | 'p'..='t' | 'v'..='z')
}

/// Convert Avro (Unicode text) into 36July (Bijoy Classic SutonnyMJ text)
pub fn avro_to_36july(avro_unicode_text: &str) -> String {
    unicode_to_classic(avro_unicode_text)
}

/// Convert 36July (Bijoy Classic SutonnyMJ text) into Avro (Unicode text)
pub fn bijoy_36july_to_avro(bijoy_classic_text: &str) -> String {
    classic_to_unicode(bijoy_classic_text)
}
