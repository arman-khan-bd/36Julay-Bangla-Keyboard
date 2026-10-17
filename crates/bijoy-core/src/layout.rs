//! Layout definitions and key mappings for Bijoy Bangla Keyboard.
//! Based on National Standard BDS 1738:2018, BDS 1935:2018, BDS 1520:2018
//! and the official "Bijoy Bangla Typing Nirdeshika".

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    Consonant(char),
    Vowel(char),
    VowelSign(char),
    Modifier(char),
    SpecialRef,      // Shift + a ('A')
    SpecialRPhala,   // 'z'
    SpecialYaPhala,  // Shift + z ('Z')
    LinkVirama,      // 'g'
    Punctuation(char),
    Digit(char),
    Passthrough(char),
}

/// Lookup the character mapped to a physical key with/without Shift.
pub fn map_key(ch: char) -> Option<KeyType> {
    match ch {
        // Digits
        '0' => Some(KeyType::Digit('০')),
        '1' => Some(KeyType::Digit('১')),
        '2' => Some(KeyType::Digit('২')),
        '3' => Some(KeyType::Digit('৩')),
        '4' => Some(KeyType::Digit('৪')),
        '5' => Some(KeyType::Digit('৫')),
        '6' => Some(KeyType::Digit('৬')),
        '7' => Some(KeyType::Digit('৭')),
        '8' => Some(KeyType::Digit('৮')),
        '9' => Some(KeyType::Digit('৯')),

        // Punctuation & symbols
        '`' => Some(KeyType::Punctuation('‘')),
        '~' => Some(KeyType::Punctuation('“')),
        '$' => Some(KeyType::Punctuation('৳')), // Shift + 4
        '&' => Some(KeyType::Modifier('ঁ')),   // Shift + 7: Chandrabindu
        '\\' => Some(KeyType::Consonant('ৎ')), // Khanda-Ta
        '|' => Some(KeyType::Modifier('ঃ')),   // Shift + \: Visarga

        // QWERTY row unshifted
        'q' => Some(KeyType::Consonant('ঙ')),
        'w' => Some(KeyType::Consonant('য')),
        'e' => Some(KeyType::Consonant('ড')),
        'r' => Some(KeyType::Consonant('প')),
        't' => Some(KeyType::Consonant('ট')),
        'y' => Some(KeyType::Consonant('চ')),
        'u' => Some(KeyType::Consonant('জ')),
        'i' => Some(KeyType::Consonant('হ')),
        'o' => Some(KeyType::Consonant('গ')),
        'p' => Some(KeyType::Consonant('ড়')),

        // QWERTY row shifted
        'Q' => Some(KeyType::Modifier('ং')),   // Anusvara
        'W' => Some(KeyType::Consonant('য়')),
        'E' => Some(KeyType::Consonant('ঢ')),
        'R' => Some(KeyType::Consonant('ফ')),
        'T' => Some(KeyType::Consonant('ঠ')),
        'Y' => Some(KeyType::Consonant('ছ')),
        'U' => Some(KeyType::Consonant('ঝ')),
        'I' => Some(KeyType::Consonant('ঞ')),
        'O' => Some(KeyType::Consonant('ঘ')),
        'P' => Some(KeyType::Consonant('ঢ়')),

        // ASDF row unshifted
        'a' => Some(KeyType::VowelSign('ৃ')),
        's' => Some(KeyType::VowelSign('ু')),
        'd' => Some(KeyType::VowelSign('ি')),
        'f' => Some(KeyType::VowelSign('া')),
        'g' => Some(KeyType::LinkVirama),
        'h' => Some(KeyType::Consonant('ব')),
        'j' => Some(KeyType::Consonant('ক')),
        'k' => Some(KeyType::Consonant('ত')),
        'l' => Some(KeyType::Consonant('দ')),

        // ASDF row shifted
        'A' => Some(KeyType::SpecialRef),       // Shift + a: Ref (র্)
        'S' => Some(KeyType::VowelSign('ূ')),
        'D' => Some(KeyType::VowelSign('ী')),
        'F' => Some(KeyType::Vowel('অ')),
        'G' => Some(KeyType::Punctuation('।')), // Shift + g: Dari
        'H' => Some(KeyType::Consonant('ভ')),
        'J' => Some(KeyType::Consonant('খ')),
        'K' => Some(KeyType::Consonant('থ')),
        'L' => Some(KeyType::Consonant('ধ')),

        // ZXCV row unshifted
        'z' => Some(KeyType::SpecialRPhala),    // 'z': r-phala (্র)
        'x' => Some(KeyType::Vowel('ও')),
        'c' => Some(KeyType::VowelSign('ে')),
        'v' => Some(KeyType::Consonant('র')),
        'b' => Some(KeyType::Consonant('ন')),
        'n' => Some(KeyType::Consonant('স')),
        'm' => Some(KeyType::Consonant('ম')),

        // ZXCV row shifted
        'Z' => Some(KeyType::SpecialYaPhala),   // Shift + z: ya-phala (্য)
        'X' => Some(KeyType::VowelSign('ৌ')),
        'C' => Some(KeyType::VowelSign('ৈ')),
        'V' => Some(KeyType::Consonant('ল')),
        'B' => Some(KeyType::Consonant('ণ')),
        'N' => Some(KeyType::Consonant('ষ')),
        'M' => Some(KeyType::Consonant('শ')),

        _ => None,
    }
}

/// Map independent vowel when 'g' (Link) is followed by a vowel sign or related key.
pub fn map_linked_vowel(ch: char) -> Option<char> {
    match ch {
        'f' => Some('আ'), // g + f (া) => আ
        'd' => Some('ই'), // g + d (ি) => ই
        'D' => Some('ঈ'), // g + D (ী) => ঈ
        's' => Some('উ'), // g + s (ু) => উ
        'S' => Some('ঊ'), // g + S (ূ) => ঊ
        'a' => Some('ঋ'), // g + a (ৃ) => ঋ
        'c' => Some('এ'), // g + c (ে) => এ
        'C' => Some('ঐ'), // g + C (ৈ) => ঐ
        'X' => Some('ঔ'), // g + X (ৌ) => ঔ
        _ => None,
    }
}

/// Map archaic Bengali, Assamese, and Vedic characters introduced in Bijoy 2024.
pub fn map_ancient_or_special(ch: char) -> Option<char> {
    match ch {
        '1' => Some('\u{09F4}'), // ৴ (Ek Ana)
        '2' => Some('\u{09F5}'), // ৵ (Dui Ana)
        '3' => Some('\u{09F6}'), // ৶ (Tin Ana)
        '4' => Some('\u{09F7}'), // ৷ (Char Ana)
        '5' => Some('\u{09F8}'), // ৸ (Aat Ana)
        '6' => Some('\u{09F9}'), // ৹ (Gonda)
        '7' => Some('\u{09FA}'), // ৺ (Ishwar)
        '8' => Some('\u{09F2}'), // ৲ (Rupee/Taka mark)
        '-' => Some('\u{09FB}'), // ৻ (Ganda)
        ',' => Some('\u{09F0}'), // ৰ (Assamese Ra)
        '.' => Some('\u{09F1}'), // ৱ (Assamese Va)
        '#' => Some('\u{09BD}'), // ঽ (Bengali Avagraha)
        'G' => Some('॥'),        // ॥ (Double Dari)
        _ => None,
    }
}
