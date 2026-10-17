//! Bengali Dictionary and Word Suggestion Engine for the Preview Window.
//! Matches prefix, phonetic variations, and similar words as shown in Avro/Bijoy preview windows.

pub static BENGALI_WORDS: &[&str] = &[
    "সোনার", "সনার", "শোনার", "শোণার", "সোনারগাঁও", "সোনালী", "সোনা",
    "আমার", "আমরা", "আমাদের", "আমায়", "আমারে", "আমাকে",
    "বাংলা", "বাংলাদেশ", "বাংলাদেশী", "বাঙালি", "বাঙলা", "বাঙালী",
    "ভালবাসি", "ভালোবাসি", "ভালো", "ভাল", "ভালোভাবে", "ভালবাসা", "ভালোবাসা",
    "চিরদিন", "চিরকাল", "চিরতরে", "চিরন্তন",
    "তোমার", "তোমরা", "তোমাদের", "তোমায়", "তোমারে", "তোমাকে",
    "আকাশ", "বাতাস", "প্রাণে", "বাজায়", "বাঁশি", "বাঁশী",
    "মা", "মাতৃভূমি", "মাটি", "মানুষ", "মন", "মনে", "মায়ের",
    "ওমা", "ফাগুনে", "আমের", "বনে", "ঘ্রাণে", "পাগল", "করে",
    "অঘ্রানে", "ভরা", "ক্ষেতে", "আমি", "কী", "দেখেছি", "মধুর", "হাসি",
    "কী", "শোভা", "ছায়া", "স্নেহ", "মমতা", "বিছায়েছ", "বটের", "মূলে", "নদীর", "কূলে",
    "বচন", "কথা", "কানে", "লাগে", "সুর", "বাজে",
    "একটি", "দুটি", "তিনটি", "চারটি", "পাঁচটি", "সকল", "সব", "সবাই",
    "নমস্কার", "সালাম", "ধন্যবাদ", "স্বাগতম", "অভিনন্দন", "শুভেচ্ছা",
    "ডিজিটাল", "কম্পিউটার", "কীবোর্ড", "সফ্টওয়্যার", "প্রোগ্রাম", "টাইপিং",
    "স্বাধীনতা", "সার্বভৌমত্ব", "গণতন্ত্র", "বিপ্লব", "জুলাই", "৩৬জুলাই", "বিজয়",
    "শহীদ", "স্মৃতি", "ইতিহাস", "সংস্কৃতি", "সাহিত্য", "কবিতা", "গান",
    "বন্ধু", "বান্ধব", "পরিবার", "সমাজ", "দেশ", "বিদেশ", "পৃথিবী",
    "বিদ্যালয়", "বিশ্ববিদ্যালয়", "কলেজ", "মাদ্রাসা", "শিক্ষক", "ছাত্র", "ছাত্রী",
    "বিজ্ঞান", "প্রযুক্তি", "গণিত", "পদার্থ", "রসায়ন", "জীববিজ্ঞান",
    "সন্ধ্যা", "সকাল", "দুপুর", "রাত", "দিন", "সময়", "কাল",
    "শুদ্ধ", "অশুদ্ধ", "সঠিক", "ভুল", "সত্য", "মিথ্যা", "জ্ঞান", "বিজ্ঞান",
    "ব্রাহ্মণ", "চিহ্ন", "আকাঙ্ক্ষা", "অঞ্জলি", "শান্তি", "আনন্দ", "সুখ", "দুঃখ",
];

/// Get word suggestions based on typed input.
/// Supports Bengali prefix/fuzzy matching and English phonetic input.
pub fn get_word_suggestions(query: &str, limit: usize) -> Vec<String> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }

    let q_lower = q.to_lowercase();
    let mut results: Vec<String> = Vec::new();

    // 1. Exact phonetic matches for English input
    if q.is_ascii() {
        match q_lower.as_str() {
            "sonar" => {
                results.push("সোনার".to_string());
                results.push("সনার".to_string());
                results.push("শোনার".to_string());
                results.push("শোণার".to_string());
            }
            "amar" => {
                results.push("আমার".to_string());
                results.push("আমরা".to_string());
                results.push("আমাদের".to_string());
                results.push("আমায়".to_string());
            }
            "bangla" => {
                results.push("বাংলা".to_string());
                results.push("বাংলাদেশ".to_string());
                results.push("বাঙালি".to_string());
                results.push("বাঙলা".to_string());
            }
            "bhalo" | "valo" => {
                results.push("ভালো".to_string());
                results.push("ভালোবাসি".to_string());
                results.push("ভাল".to_string());
                results.push("ভালবাসি".to_string());
            }
            "tomar" => {
                results.push("তোমার".to_string());
                results.push("তোমরা".to_string());
                results.push("তোমাদের".to_string());
                results.push("তোমায়".to_string());
            }
            "ami" => {
                results.push("আমি".to_string());
                results.push("আমায়".to_string());
                results.push("আমরা".to_string());
            }
            "tumi" => {
                results.push("তুমি".to_string());
                results.push("তোমরা".to_string());
                results.push("তোমাকে".to_string());
            }
            "desh" => {
                results.push("দেশ".to_string());
                results.push("দেশের".to_string());
                results.push("দেশবাসী".to_string());
                results.push("বিদেশ".to_string());
            }
            "bijoy" => {
                results.push("বিজয়".to_string());
                results.push("বিজয়ী".to_string());
                results.push("বিজয় দিবস".to_string());
                results.push("বিজয় ৩৬জুলাই".to_string());
            }
            _ => {}
        }
    }

    // 2. Direct Bengali prefix matching
    for &word in BENGALI_WORDS {
        if word.starts_with(q) {
            if !results.iter().any(|r| r == word) {
                results.push(word.to_string());
            }
        }
        if results.len() >= limit {
            break;
        }
    }

    // 3. Similar spelling variations (e.g., s/sh/S: স/শ/ষ, n/N: ন/ণ)
    if results.len() < limit {
        let normalized_q = normalize_bengali_phonetic(q);
        for &word in BENGALI_WORDS {
            let normalized_word = normalize_bengali_phonetic(word);
            if normalized_word.starts_with(&normalized_q) || normalized_word.contains(&normalized_q) {
                if !results.iter().any(|r| r == word) {
                    results.push(word.to_string());
                }
            }
            if results.len() >= limit {
                break;
            }
        }
    }

    results
}

fn normalize_bengali_phonetic(s: &str) -> String {
    s.replace(['শ', 'ষ', 'স'], "স")
     .replace(['ণ', 'ন'], "ন")
     .replace(['ী', 'ি'], "ি")
     .replace(['ূ', 'ু'], "ু")
     .replace(['ড়', 'ঢ়', 'র'], "র")
     .replace(['য', 'জ'], "জ")
}
