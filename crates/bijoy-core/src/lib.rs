//! Bijoy Bangla Keyboard Core Library
//! Accurate implementation of the Bijoy keyboard layout as defined in BDS standards
//! and the official Bijoy Typing Guide.

pub mod avro;
pub mod converter;
pub mod dictionary;
pub mod engine;
pub mod layout;
pub mod ligatures;

pub use avro::{avro_phonetic_to_unicode, avro_to_36july, bijoy_36july_to_avro};
pub use converter::{classic_to_unicode, unicode_to_classic};
pub use dictionary::get_word_suggestions;
pub use engine::{BijoyEngine, BijoyMode, EngineOutput};
pub use layout::{map_ancient_or_special, map_key, map_linked_vowel, KeyType};
pub use ligatures::{LigatureInfo, LIGATURES};

#[cfg(test)]
mod tests {
    use super::*;

    fn type_string(engine: &mut BijoyEngine, input: &str) -> String {
        let mut text = String::new();
        for ch in input.chars() {
            let out = engine.process_char(ch);
            if out.delete_count > 0 {
                for _ in 0..out.delete_count {
                    text.pop();
                }
            }
            text.push_str(&out.insert_text);
        }
        text
    }

    #[test]
    fn test_independent_vowels() {
        let mut engine = BijoyEngine::new();

        assert_eq!(type_string(&mut engine, "F"), "অ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gf"), "আ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gd"), "ই");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gD"), "ঈ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gs"), "উ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gS"), "ঊ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "ga"), "ঋ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gc"), "এ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gC"), "ঐ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "x"), "ও");
        engine.reset();
        assert_eq!(type_string(&mut engine, "gX"), "ঔ");
    }

    #[test]
    fn test_consonants() {
        let mut engine = BijoyEngine::new();
        assert_eq!(type_string(&mut engine, "j"), "ক");
        engine.reset();
        assert_eq!(type_string(&mut engine, "J"), "খ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "o"), "গ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "O"), "ঘ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "q"), "ঙ");
        engine.reset();
        assert_eq!(type_string(&mut engine, "v"), "র");
        engine.reset();
        assert_eq!(type_string(&mut engine, "V"), "ল");
        engine.reset();
        assert_eq!(type_string(&mut engine, "h"), "ব");
        engine.reset();
        assert_eq!(type_string(&mut engine, "H"), "ভ");
    }

    #[test]
    fn test_ligatures_and_words() {
        let mut engine = BijoyEngine::new();

        // ক্ত (j + g + k)
        assert_eq!(type_string(&mut engine, "jgk"), "ক্ত");
        engine.reset();

        // ক্ষ (j + g + N)
        assert_eq!(type_string(&mut engine, "jgN"), "ক্ষ");
        engine.reset();

        // জ্ঞ (u + g + I)
        assert_eq!(type_string(&mut engine, "ugI"), "জ্ঞ");
        engine.reset();

        // বাংলা (h + f + Q + V + f => ব + া + ং + ল + া)
        assert_eq!(type_string(&mut engine, "hfQVf"), "বাংলা");
        engine.reset();

        // বাংলাদেশ (h + f + Q + V + f + c + l + M => ব + া + ং + ল + া + ে + দ + শ)
        assert_eq!(type_string(&mut engine, "hfQVfclM"), "বাংলাদেশ");
        engine.reset();
    }

    #[test]
    fn test_ref_and_phala() {
        let mut engine = BijoyEngine::new();

        // ক্র (j + z)
        assert_eq!(type_string(&mut engine, "jz"), "ক্র");
        engine.reset();

        // ক্য (j + Z)
        assert_eq!(type_string(&mut engine, "jZ"), "ক্য");
        engine.reset();

        // র্ক (A + j or v + g + j)
        assert_eq!(type_string(&mut engine, "Aj"), "র্ক");
        engine.reset();
        assert_eq!(type_string(&mut engine, "vgj"), "র্ক");
    }

    #[test]
    fn test_complex_words_from_pdf() {
        let mut engine = BijoyEngine::new();

        // বিজ্ঞান (d + h + ugI + f + b => বি + জ্ঞ + া + ন)
        assert_eq!(type_string(&mut engine, "dhugIfb"), "বিজ্ঞান");
        engine.reset();

        // ব্রাহ্মণ (h + z + f + igm + B => ব্র + া + হ্ম + ণ)
        assert_eq!(type_string(&mut engine, "hzfigmB"), "ব্রাহ্মণ");
        engine.reset();

        // চিহ্ন (d + y + igb => চি + হ্ন)
        assert_eq!(type_string(&mut engine, "dyigb"), "চিহ্ন");
        engine.reset();

        // শুদ্ধ (M + s + lgL => শু + দ্ধ)
        assert_eq!(type_string(&mut engine, "MslgL"), "শুদ্ধ");
        engine.reset();

        // আকাঙ্ক্ষা (gf + j + f + qgjgN + f => আ + কা + ঙ্ক্ষ + া)
        assert_eq!(type_string(&mut engine, "gfjfqgjgNf"), "আকাঙ্ক্ষা");
        engine.reset();

        // অঞ্জলি (F + Iqu + d + V => অ + ঞ্জ + লি)
        assert_eq!(type_string(&mut engine, "FIgudV"), "অঞ্জলি");
        engine.reset();

        // ও-কার (c + k + f => তে + া => তো)
        assert_eq!(type_string(&mut engine, "ckf"), "তো");
        engine.reset();

        // ঔ-কার (c + k + X => তে + ৌ => তৌ)
        assert_eq!(type_string(&mut engine, "ckX"), "তৌ");
        engine.reset();
    }

    #[test]
    fn test_converter_bidirectional() {
        let classic_sample = "wWwRUvj evsjv‡`k";
        let unicode = classic_to_unicode(classic_sample);
        assert_eq!(unicode, "ডিজিটাল বাংলাদেশ");

        let converted_back = unicode_to_classic(&unicode);
        assert_eq!(classic_to_unicode(&converted_back), unicode);
    }

    #[test]
    fn test_suggestions_like_image_2() {
        let suggestions = get_word_suggestions("sOnar", 4);
        assert_eq!(suggestions, vec!["সোনার", "সনার", "শোনার", "শোণার"]);
    }

    #[test]
    fn test_avro_to_36july_converter() {
        let avro_unicode = "ডিজিটাল বাংলাদেশ";
        let bijoy_classic = avro_to_36july(avro_unicode);
        assert_eq!(bijoy_36july_to_avro(&bijoy_classic), avro_unicode);
    }

    #[test]
    fn test_zero_backspace_pre_kars() {
        let mut engine = BijoyEngine::new();
        // dk (তি) - zero backspaces!
        assert_eq!(type_string(&mut engine, "dk"), "তি");
        engine.reset();
        // gfdm (আমি)
        assert_eq!(type_string(&mut engine, "gfdm"), "আমি");
        engine.reset();
        // clM (দেশ: c=ে, l=দ, M=শ)
        assert_eq!(type_string(&mut engine, "clM"), "দেশ");
        engine.reset();
        // dkdm (তিমি)
        assert_eq!(type_string(&mut engine, "dkdm"), "তিমি");
        engine.reset();
    }

    #[test]
    fn test_avro_mode_typing() {
        let mut engine = BijoyEngine::new();
        engine.set_mode(BijoyMode::Avro);

        assert_eq!(type_string(&mut engine, "ami "), "আমি ");
        assert_eq!(type_string(&mut engine, "sonar "), "সোনার ");
        assert_eq!(type_string(&mut engine, "bangla "), "বাংলা ");
    }

    #[test]
    fn test_preview_and_space_hide() {
        let mut engine = BijoyEngine::new();
        engine.set_mode(BijoyMode::Avro);

        // When user types letters in Avro:
        engine.process_char('s');
        engine.process_char('o');
        engine.process_char('n');
        engine.process_char('a');
        engine.process_char('r');

        let (query, sugs) = engine.get_preview_info();
        assert_eq!(query, "sonar");
        assert!(!sugs.is_empty());
        assert_eq!(sugs[0], "সোনার");

        // When space is pressed: word is committed and preview buffer is cleared!
        let out = engine.process_char(' ');
        assert_eq!(out.insert_text, "সোনার ");
        let (query_after_space, sugs_after_space) = engine.get_preview_info();
        assert!(query_after_space.is_empty());
        assert!(sugs_after_space.is_empty());

        // In Unicode mode:
        engine.set_mode(BijoyMode::Unicode);
        engine.process_char('g');
        engine.process_char('f'); // আ
        engine.process_char('m'); // ম
        engine.process_char('f'); // া
        let (q_uni, s_uni) = engine.get_preview_info();
        assert_eq!(q_uni, "আমা");
        assert!(!s_uni.is_empty());

        // Space clears Unicode word buffer:
        engine.process_char(' ');
        let (q_after, _) = engine.get_preview_info();
        assert!(q_after.is_empty());
    }

    #[test]
    fn test_select_suggestion() {
        let mut engine = BijoyEngine::new();
        engine.set_mode(BijoyMode::Avro);
        for c in "sonar".chars() {
            engine.process_char(c);
        }
        let sel = engine.select_suggestion(0);
        assert_eq!(sel, Some((5, "সোনার ".to_string())));
        let (q, _) = engine.get_preview_info();
        assert!(q.is_empty());
    }
}
