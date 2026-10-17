//! Bidirectional converter between Bijoy Classic (SutonnyMJ / ANSI) and Bengali Unicode.
//! Essential for publishing, DTP, Word documents, and online migration.

use crate::ligatures::LIGATURES;

/// Convert Bijoy Classic (SutonnyMJ ANSI text) to standard Bengali Unicode.
pub fn classic_to_unicode(input: &str) -> String {
    let mut text = input.to_string();

    // 1. Independent Vowels
    text = text.replace("Av", "আ");
    text = text.replace("A", "অ");
    text = text.replace("B", "ই");
    text = text.replace("C", "ঈ");
    text = text.replace("D", "উ");
    text = text.replace("E", "ঊ");
    text = text.replace("F", "ঋ");
    text = text.replace("G", "এ");
    text = text.replace("H", "ঐ");
    text = text.replace("I", "ও");
    text = text.replace("J", "ঔ");

    // 2. Multi-char & single-char ligatures
    // Sort by length descending to match longest sequences first
    let mut sorted_ligs = LIGATURES.to_vec();
    sorted_ligs.sort_by(|a, b| b.sutonny.len().cmp(&a.sutonny.len()));
    for lig in &sorted_ligs {
        text = text.replace(lig.sutonny, lig.unicode);
    }

    // 3. Consonants
    text = text.replace("K", "ক");
    text = text.replace("L", "খ");
    text = text.replace("M", "গ");
    text = text.replace("N", "ঘ");
    text = text.replace("O", "ঙ");
    text = text.replace("P", "চ");
    text = text.replace("Q", "ছ");
    text = text.replace("R", "জ");
    text = text.replace("S", "ঝ");
    text = text.replace("T", "ঞ");
    text = text.replace("U", "ট");
    text = text.replace("V", "ঠ");
    text = text.replace("W", "ড");
    text = text.replace("X", "ঢ");
    text = text.replace("Y", "ণ");
    text = text.replace("Z", "ত");
    text = text.replace("_", "থ");
    text = text.replace("`", "দ");
    text = text.replace("a", "ধ");
    text = text.replace("b", "ন");
    text = text.replace("c", "প");
    text = text.replace("d", "ফ");
    text = text.replace("e", "ব");
    text = text.replace("f", "ভ");
    text = text.replace("g", "ম");
    text = text.replace("h", "য");
    text = text.replace("i", "র");
    text = text.replace("j", "ল");
    text = text.replace("k", "শ");
    text = text.replace("l", "ষ");
    text = text.replace("m", "স");
    text = text.replace("n", "হ");
    text = text.replace("o", "ড়");
    text = text.replace("p", "ঢ়");
    text = text.replace("q", "য়");
    text = text.replace("r", "ৎ");
    text = text.replace("s", "ং");
    text = text.replace("t", "ঃ");
    text = text.replace("u", "ঁ");
    text = text.replace("|", "।");
    text = text.replace("&", "্");

    // 4. Post-kars, alternative glyphs, and modifiers
    text = text.replace("ÿ", "ক্ষ");
    text = text.replace("v", "া");
    text = text.replace("x", "ী");
    text = text.replace("y", "ু");
    text = text.replace("z", "ু"); // alternative u-kar
    text = text.replace("~", "ূ");
    text = text.replace("‚", "ূ"); // alternative oo-kar
    text = text.replace("„", "ৃ");
    text = text.replace("Š", "ৌ");
    text = text.replace("ª", "্র");
    text = text.replace("¨", "্য");
    text = text.replace("Ò", "“");
    text = text.replace("Ó", "”");
    text = text.replace("Ô", "‘");
    text = text.replace("Õ", "’");

    // 5. Reorder Pre-kars: '†' (ে), 'w' (ি), 'ˆ' (ৈ)
    // In Sutonny, pre-kar is before consonant or cluster.
    // In Unicode, it must be placed after the cluster.
    // Also: '†' + consonant + 'া' => consonant + 'ো'
    text = reorder_sutonny_prekars(&text);

    // 6. Ref reordering: '©' in Sutonny is after consonant,
    // in Unicode Ref is "র্" before the consonant/cluster.
    text = reorder_sutonny_ref(&text);

    text
}

/// Helper to reorder pre-kars from Sutonny to Unicode
fn reorder_sutonny_prekars(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch == '†' || ch == '‡' || ch == 'w' || ch == 'ˆ' {
            let pre_kar = match ch {
                '†' | '‡' => 'ে',
                'w' => 'ি',
                'ˆ' => 'ৈ',
                _ => unreachable!(),
            };
            i += 1;

            // Find full consonant cluster following the pre-kar.
            // A cluster starts with a consonant, followed by zero or more ('্' + consonant).
            let mut cluster = String::new();
            if i < chars.len() && is_bengali_consonant(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
                while i + 1 < chars.len() && chars[i] == '্' && is_bengali_consonant(chars[i + 1]) {
                    cluster.push(chars[i]);
                    cluster.push(chars[i + 1]);
                    i += 2;
                }
            }

            if !cluster.is_empty() {
                // Check if followed by 'া' to make 'ো'
                if pre_kar == 'ে' && i < chars.len() && chars[i] == 'া' {
                    result.push_str(&cluster);
                    result.push('ো');
                    i += 1;
                } else if pre_kar == 'ে' && i < chars.len() && chars[i] == 'ৌ' {
                    result.push_str(&cluster);
                    result.push('ৌ');
                    i += 1;
                } else {
                    result.push_str(&cluster);
                    result.push(pre_kar);
                }
            } else {
                result.push(pre_kar);
            }
        } else {
            result.push(ch);
            i += 1;
        }
    }

    result
}

/// Helper to reorder Ref '©' in Sutonny to Unicode "র্"
fn reorder_sutonny_ref(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '©' {
            // Find consonant/cluster immediately preceding the ref
            // In Sutonny, Ref is placed after the consonant, e.g. "K©" => র্ক
            // So we need to insert 'র' + '্' before that preceding consonant
            let r_chars: Vec<char> = result.chars().collect();
            let mut start = r_chars.len();
            while start > 0 {
                let c = r_chars[start - 1];
                if is_bengali_consonant(c) || c == '্' {
                    start -= 1;
                } else {
                    break;
                }
            }
            if start < r_chars.len() {
                let before: String = r_chars[..start].iter().collect();
                let cons: String = r_chars[start..].iter().collect();
                result = format!("{}র্{}", before, cons);
            } else {
                result.push_str("র্");
            }
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Convert Bengali Unicode to Bijoy Classic (SutonnyMJ ANSI).
pub fn unicode_to_classic(input: &str) -> String {
    let mut text = input.to_string();

    // 1. Reorder O-kar ('ো') => '†' + Consonant + 'v'
    // and Ou-kar ('ৌ') => '†' + Consonant + 'Š'
    text = reorder_unicode_okars(&text);

    // 2. Reorder Pre-kars: 'ে' => '†', 'ি' => 'w', 'ৈ' => 'ˆ'
    // In Unicode: Consonant + 'ে' => In Sutonny: '†' + Consonant
    text = reorder_unicode_prekars(&text);

    // 3. Move Ref ("র্" = 'র' + '্') after consonant/cluster to '©'
    text = reorder_unicode_ref(&text);

    // 4. Convert ligatures
    let mut sorted_ligs = LIGATURES.to_vec();
    sorted_ligs.sort_by(|a, b| b.unicode.len().cmp(&a.unicode.len()));
    for lig in &sorted_ligs {
        text = text.replace(lig.unicode, lig.sutonny);
    }

    // 5. Independent vowels
    text = text.replace("আ", "Av");
    text = text.replace("অ", "A");
    text = text.replace("ই", "B");
    text = text.replace("ঈ", "C");
    text = text.replace("উ", "D");
    text = text.replace("ঊ", "E");
    text = text.replace("ঋ", "F");
    text = text.replace("এ", "G");
    text = text.replace("ঐ", "H");
    text = text.replace("ও", "I");
    text = text.replace("ঔ", "J");

    // 6. Consonants
    text = text.replace("ক", "K");
    text = text.replace("খ", "L");
    text = text.replace("গ", "M");
    text = text.replace("ঘ", "N");
    text = text.replace("ঙ", "O");
    text = text.replace("চ", "P");
    text = text.replace("ছ", "Q");
    text = text.replace("জ", "R");
    text = text.replace("ঝ", "S");
    text = text.replace("ঞ", "T");
    text = text.replace("ট", "U");
    text = text.replace("ঠ", "V");
    text = text.replace("ড", "W");
    text = text.replace("ঢ", "X");
    text = text.replace("ণ", "Y");
    text = text.replace("ত", "Z");
    text = text.replace("থ", "_");
    text = text.replace("দ", "`");
    text = text.replace("ধ", "a");
    text = text.replace("ন", "b");
    text = text.replace("প", "c");
    text = text.replace("ফ", "d");
    text = text.replace("ব", "e");
    text = text.replace("ভ", "f");
    text = text.replace("ম", "g");
    text = text.replace("য", "h");
    text = text.replace("র", "i");
    text = text.replace("ল", "j");
    text = text.replace("শ", "k");
    text = text.replace("ষ", "l");
    text = text.replace("স", "m");
    text = text.replace("হ", "n");
    text = text.replace("ড়", "o");
    text = text.replace("ঢ়", "p");
    text = text.replace("য়", "q");
    text = text.replace("ৎ", "r");
    text = text.replace("ং", "s");
    text = text.replace("ঃ", "t");
    text = text.replace("ঁ", "u");
    text = text.replace("।", "|");
    text = text.replace("্", "&");

    // 7. Post-kars
    text = text.replace("া", "v");
    text = text.replace("ী", "x");
    text = text.replace("ু", "y");
    text = text.replace("ূ", "~");
    text = text.replace("ৃ", "„");
    text = text.replace("ৌ", "Š");

    text
}

fn find_preceding_cluster_start(r_chars: &[char]) -> usize {
    if r_chars.is_empty() {
        return 0;
    }
    let mut curr = r_chars.len();
    if curr > 0 && is_bengali_consonant(r_chars[curr - 1]) {
        curr -= 1;
        while curr >= 2 && r_chars[curr - 1] == '্' && is_bengali_consonant(r_chars[curr - 2]) {
            curr -= 2;
        }
        curr
    } else {
        r_chars.len()
    }
}

fn reorder_unicode_okars(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch == 'ো' || ch == 'ৌ' {
            let post_kar = if ch == 'ো' { 'v' } else { 'Š' };
            let r_chars: Vec<char> = result.chars().collect();
            let start = find_preceding_cluster_start(&r_chars);
            if start < r_chars.len() {
                let before: String = r_chars[..start].iter().collect();
                let cons: String = r_chars[start..].iter().collect();
                result = format!("{}†{}{}", before, cons, post_kar);
            } else {
                result.push_str("†");
                result.push(post_kar);
            }
            i += 1;
        } else {
            result.push(ch);
            i += 1;
        }
    }

    result
}

fn reorder_unicode_prekars(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch == 'ে' || ch == 'ি' || ch == 'ৈ' {
            let pre_char = match ch {
                'ে' => '†',
                'ি' => 'w',
                'ৈ' => 'ˆ',
                _ => unreachable!(),
            };
            let r_chars: Vec<char> = result.chars().collect();
            let start = find_preceding_cluster_start(&r_chars);
            if start < r_chars.len() {
                let before: String = r_chars[..start].iter().collect();
                let cons: String = r_chars[start..].iter().collect();
                result = format!("{}{}{}", before, pre_char, cons);
            } else {
                result.push(pre_char);
            }
            i += 1;
        } else {
            result.push(ch);
            i += 1;
        }
    }

    result
}

fn reorder_unicode_ref(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == 'র'
            && i + 1 < chars.len()
            && chars[i + 1] == '্'
            && i + 2 < chars.len()
            && is_bengali_consonant(chars[i + 2])
        {
            i += 2; // skip 'র' and '্'
            let mut cluster = String::new();
            while i < chars.len() {
                let c = chars[i];
                if is_bengali_consonant(c) || c == '্' {
                    cluster.push(c);
                    i += 1;
                } else {
                    break;
                }
            }
            result.push_str(&cluster);
            result.push('©');
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

fn is_bengali_consonant(c: char) -> bool {
    matches!(c, '\u{0995}'..='\u{09B9}' | '\u{09DC}'..='\u{09DF}' | '\u{09CE}')
}
