//! Ligature tables and mapping rules for Bijoy Bangla (Unicode and Classic/SutonnyMJ).
//! Directly derived from the official Bijoy Typing Guide (pages 6-11).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LigatureInfo {
    pub name: &'static str,
    pub unicode: &'static str,
    pub keys: &'static str,
    pub sutonny: &'static str,
}

pub static LIGATURES: &[LigatureInfo] = &[
    // ক ligatures
    LigatureInfo { name: "ক্ক", unicode: "ক্ক", keys: "jgj", sutonny: "\u{00B0}" },
    LigatureInfo { name: "ক্ট", unicode: "ক্ট", keys: "jgt", sutonny: "\u{00B1}" },
    LigatureInfo { name: "ক্ন", unicode: "ক্ন", keys: "jgb", sutonny: "Kè" },
    LigatureInfo { name: "ক্ব", unicode: "ক্ব", keys: "jgh", sutonny: "K¡" },
    LigatureInfo { name: "ক্ম", unicode: "ক্ম", keys: "jgm", sutonny: "´" },
    LigatureInfo { name: "ক্র", unicode: "ক্র", keys: "jz",  sutonny: "\u{00B5}" },
    LigatureInfo { name: "ক্ল", unicode: "ক্ল", keys: "jgV", sutonny: "K¬" },
    LigatureInfo { name: "ক্ষ", unicode: "ক্ষ", keys: "jgN", sutonny: "\u{00B2}" },
    LigatureInfo { name: "ক্ষ্ণ", unicode: "ক্ষ্ণ", keys: "jgNgB", sutonny: "ÿè" },
    LigatureInfo { name: "ক্ষ্ম", unicode: "ক্ষ্ম", keys: "jgNgm", sutonny: "²" },
    LigatureInfo { name: "ক্স", unicode: "ক্স", keys: "jgn", sutonny: "·" },
    LigatureInfo { name: "ক্ত", unicode: "ক্ত", keys: "jgk", sutonny: "\u{00B3}" },

    // খ ligatures
    LigatureInfo { name: "খ্র", unicode: "খ্র", keys: "Jz", sutonny: "Lª" },

    // গ ligatures
    LigatureInfo { name: "গ্দ", unicode: "গ্দ", keys: "ogl", sutonny: "\u{00BA}" },
    LigatureInfo { name: "গ্ধ", unicode: "গ্ধ", keys: "ogL", sutonny: "\u{00BB}" },
    LigatureInfo { name: "গ্ন", unicode: "গ্ন", keys: "ogb", sutonny: "Mœ" },
    LigatureInfo { name: "গ্ম", unicode: "গ্ম", keys: "ogm", sutonny: "M¥" },
    LigatureInfo { name: "গ্র", unicode: "গ্র", keys: "oz",  sutonny: "MÖ" },
    LigatureInfo { name: "গ্ব", unicode: "গ্ব", keys: "ogh", sutonny: "M¦" },
    LigatureInfo { name: "গ্ল", unicode: "গ্ল", keys: "ogV", sutonny: "Mø" },

    // ঘ ligatures
    LigatureInfo { name: "ঘ্ন", unicode: "ঘ্ন", keys: "Ogb", sutonny: "Nœ" },
    LigatureInfo { name: "ঘ্র", unicode: "ঘ্র", keys: "Oz",  sutonny: "Nª" },

    // ঙ ligatures
    LigatureInfo { name: "ঙ্ক", unicode: "ঙ্ক", keys: "qgj", sutonny: "\u{00BC}" },
    LigatureInfo { name: "ঙ্খ", unicode: "ঙ্খ", keys: "qgJ", sutonny: "•L" },
    LigatureInfo { name: "ঙ্গ", unicode: "ঙ্গ", keys: "qgo", sutonny: "\u{00BD}" },
    LigatureInfo { name: "ঙ্ঘ", unicode: "ঙ্ঘ", keys: "qgO", sutonny: "•N" },
    LigatureInfo { name: "ঙ্ম", unicode: "ঙ্ম", keys: "qgm", sutonny: "•g" },
    LigatureInfo { name: "ঙ্ক্ষ", unicode: "ঙ্ক্ষ", keys: "qgjgN", sutonny: "•¶" },

    // চ ligatures
    LigatureInfo { name: "চ্চ", unicode: "চ্চ", keys: "ygy", sutonny: "”P" },
    LigatureInfo { name: "চ্ছ", unicode: "চ্ছ", keys: "ygY", sutonny: "”Q" },
    LigatureInfo { name: "চ্ঞ", unicode: "চ্ঞ", keys: "ygI", sutonny: "”T" },
    LigatureInfo { name: "চ্ছ্ব", unicode: "চ্ছ্ব", keys: "ygygh", sutonny: "”Q¡" },

    // ছ ligatures
    LigatureInfo { name: "ছ্র", unicode: "ছ্র", keys: "Yz", sutonny: "Q«" },

    // জ ligatures
    LigatureInfo { name: "জ্জ", unicode: "জ্জ", keys: "ugu", sutonny: "\u{00BE}" },
    LigatureInfo { name: "জ্ঝ", unicode: "জ্ঝ", keys: "ugU", sutonny: "\u{00C0}" },
    LigatureInfo { name: "জ্ঞ", unicode: "জ্ঞ", keys: "ugI", sutonny: "\u{00C1}" },
    LigatureInfo { name: "জ্ব", unicode: "জ্ব", keys: "ugh", sutonny: "R¡" },
    LigatureInfo { name: "জ্র", unicode: "জ্র", keys: "uz",  sutonny: "Rª" },

    // ঞ ligatures
    LigatureInfo { name: "ঞ্চ", unicode: "ঞ্চ", keys: "Igy", sutonny: "\u{00C2}" },
    LigatureInfo { name: "ঞ্ছ", unicode: "ঞ্ছ", keys: "IgY", sutonny: "\u{00C3}" },
    LigatureInfo { name: "ঞ্জ", unicode: "ঞ্জ", keys: "Igu", sutonny: "\u{00C4}" },
    LigatureInfo { name: "ঞ্ঝ", unicode: "ঞ্ঝ", keys: "IgU", sutonny: "\u{00C5}" },

    // ট ligatures
    LigatureInfo { name: "ট্ট", unicode: "ট্ট", keys: "tgt", sutonny: "\u{00C6}" },
    LigatureInfo { name: "ট্ব", unicode: "ট্ব", keys: "tgh", sutonny: "U¡" },
    LigatureInfo { name: "ট্ম", unicode: "ট্ম", keys: "tgm", sutonny: "U¥" },
    LigatureInfo { name: "ট্র", unicode: "ট্র", keys: "tz",  sutonny: "Uª" },

    // ঠ ligatures
    LigatureInfo { name: "ঠ্ঠ", unicode: "ঠ্ঠ", keys: "TgT", sutonny: "Vª" },

    // ড ligatures
    LigatureInfo { name: "ড্ড", unicode: "ড্ড", keys: "ege", sutonny: "\u{00C7}" },
    LigatureInfo { name: "ড্র", unicode: "ড্র", keys: "ez",  sutonny: "Wª" },

    // ণ ligatures
    LigatureInfo { name: "ণ্ট", unicode: "ণ্ট", keys: "Bgt", sutonny: "\u{00C8}" },
    LigatureInfo { name: "ণ্ঠ", unicode: "ণ্ঠ", keys: "BgT", sutonny: "\u{00C9}" },
    LigatureInfo { name: "ণ্ড", unicode: "ণ্ড", keys: "Bge", sutonny: "\u{00D0}" },
    LigatureInfo { name: "ণ্ণ", unicode: "ণ্ণ", keys: "BgB", sutonny: "Yœ" },
    LigatureInfo { name: "ণ্ব", unicode: "ণ্ব", keys: "Bgh", sutonny: "Y¦" },
    LigatureInfo { name: "ণ্ম", unicode: "ণ্ম", keys: "Bgm", sutonny: "Y¥" },

    // ত ligatures
    LigatureInfo { name: "ত্ত", unicode: "ত্ত", keys: "kgk", sutonny: "\u{00CB}" },
    LigatureInfo { name: "ত্থ", unicode: "ত্থ", keys: "kgK", sutonny: "\u{00CC}" },
    LigatureInfo { name: "ত্ন", unicode: "ত্ন", keys: "kgb", sutonny: "Zœ" },
    LigatureInfo { name: "ত্ব", unicode: "ত্ব", keys: "kgh", sutonny: "Z¡" },
    LigatureInfo { name: "ত্ম", unicode: "ত্ম", keys: "kgm", sutonny: "Z¥" },
    LigatureInfo { name: "ত্র", unicode: "ত্র", keys: "kz",  sutonny: "\u{00CE}" },

    // থ ligatures
    LigatureInfo { name: "থ্ব", unicode: "থ্ব", keys: "Kgh", sutonny: "_¡" },
    LigatureInfo { name: "থ্র", unicode: "থ্র", keys: "Kz",  sutonny: "_ª" },

    // দ ligatures
    LigatureInfo { name: "দ্গ", unicode: "দ্গ", keys: "lgo", sutonny: "˜M" },
    LigatureInfo { name: "দ্ঘ", unicode: "দ্ঘ", keys: "lgO", sutonny: "˜N" },
    LigatureInfo { name: "দ্দ", unicode: "দ্দ", keys: "lgl", sutonny: "\u{00CF}" },
    LigatureInfo { name: "দ্ধ", unicode: "দ্ধ", keys: "lgL", sutonny: "\u{00D7}" },
    LigatureInfo { name: "দ্ব", unicode: "দ্ব", keys: "lgh", sutonny: "\u{00D8}" },
    LigatureInfo { name: "দ্ভ", unicode: "দ্ভ", keys: "lgH", sutonny: "™¢" },
    LigatureInfo { name: "দ্ম", unicode: "দ্ম", keys: "lgm", sutonny: "Ù" },
    LigatureInfo { name: "দ্র", unicode: "দ্র", keys: "lz",  sutonny: "`ª" },

    // ধ ligatures
    LigatureInfo { name: "ধ্ন", unicode: "ধ্ন", keys: "Lgb", sutonny: "aœ" },
    LigatureInfo { name: "ধ্ব", unicode: "ধ্ব", keys: "Lgh", sutonny: "aŸ" },
    LigatureInfo { name: "ধ্ম", unicode: "ধ্ম", keys: "Lgm", sutonny: "a¥" },
    LigatureInfo { name: "ধ্র", unicode: "ধ্র", keys: "Lz",  sutonny: "aª" },

    // ন ligatures
    LigatureInfo { name: "ন্ট", unicode: "ন্ট", keys: "bgt", sutonny: "›U" },
    LigatureInfo { name: "ন্ঠ", unicode: "ন্ঠ", keys: "bgT", sutonny: "Ú" },
    LigatureInfo { name: "ন্ড", unicode: "ন্ড", keys: "bge", sutonny: "Û" },
    LigatureInfo { name: "ন্ত", unicode: "ন্ত", keys: "bgk", sutonny: "šÍ" },
    LigatureInfo { name: "ন্ত্ব", unicode: "ন্ত্ব", keys: "bgkgh", sutonny: "šÍ¡" },
    LigatureInfo { name: "ন্থ", unicode: "ন্থ", keys: "bgK", sutonny: "š’" },
    LigatureInfo { name: "ন্দ", unicode: "ন্দ", keys: "bgl", sutonny: "›`" },
    LigatureInfo { name: "ন্ধ", unicode: "ন্ধ", keys: "bgL", sutonny: "Ü" },
    LigatureInfo { name: "ন্ন", unicode: "ন্ন", keys: "bgb", sutonny: "bœ" },
    LigatureInfo { name: "ন্ব", unicode: "ন্ব", keys: "bgh", sutonny: "š^" },
    LigatureInfo { name: "ন্ম", unicode: "ন্ম", keys: "bgm", sutonny: "b¥" },
    LigatureInfo { name: "ন্স", unicode: "ন্স", keys: "bgn", sutonny: "Ý" },

    // প ligatures
    LigatureInfo { name: "প্ট", unicode: "প্ট", keys: "rgt", sutonny: "Þ" },
    LigatureInfo { name: "প্ত", unicode: "প্ত", keys: "rgk", sutonny: "ß" },
    LigatureInfo { name: "প্ন", unicode: "প্ন", keys: "rgb", sutonny: "cœ" },
    LigatureInfo { name: "প্প", unicode: "প্প", keys: "rgr", sutonny: "à" },
    LigatureInfo { name: "প্র", unicode: "প্র", keys: "rz",  sutonny: "cÖ" },
    LigatureInfo { name: "প্ল", unicode: "প্ল", keys: "rgV", sutonny: "cø" },
    LigatureInfo { name: "প্স", unicode: "প্স", keys: "rgn", sutonny: "c·" },

    // ফ ligatures
    LigatureInfo { name: "ফ্র", unicode: "ফ্র", keys: "Rz",  sutonny: "d«" },
    LigatureInfo { name: "ফ্ল", unicode: "ফ্ল", keys: "RgV", sutonny: "d¬" },

    // ব ligatures
    LigatureInfo { name: "ব্জ", unicode: "ব্জ", keys: "hgu", sutonny: "â" },
    LigatureInfo { name: "ব্দ", unicode: "ব্দ", keys: "hgl", sutonny: "ã" },
    LigatureInfo { name: "ব্ধ", unicode: "ব্ধ", keys: "hgL", sutonny: "ä" },
    LigatureInfo { name: "ব্ব", unicode: "ব্ব", keys: "hgh", sutonny: "eŸ" },
    LigatureInfo { name: "ব্র", unicode: "ব্র", keys: "hz",  sutonny: "eª" },
    LigatureInfo { name: "ব্ল", unicode: "ব্ল", keys: "hgV", sutonny: "eø" },

    // ভ ligatures
    LigatureInfo { name: "ভ্র", unicode: "ভ্র", keys: "Hz", sutonny: "å" },
    LigatureInfo { name: "ভ্ল", unicode: "ভ্ল", keys: "HgV", sutonny: "fø" },

    // ম ligatures
    LigatureInfo { name: "ম্ন", unicode: "ম্ন", keys: "mgb", sutonny: "gœ" },
    LigatureInfo { name: "ম্প", unicode: "ম্প", keys: "mgr", sutonny: "¤ú" },
    LigatureInfo { name: "ম্ফ", unicode: "ম্ফ", keys: "mgR", sutonny: "ç" },
    LigatureInfo { name: "ম্ব", unicode: "ম্ব", keys: "mgh", sutonny: "¤^" },
    LigatureInfo { name: "ম্ভ", unicode: "ম্ভ", keys: "mgH", sutonny: "¤¢" },
    LigatureInfo { name: "ম্ম", unicode: "ম্ম", keys: "mgm", sutonny: "¤§" },
    LigatureInfo { name: "ম্র", unicode: "ম্র", keys: "mz",  sutonny: "gª" },
    LigatureInfo { name: "ম্ল", unicode: "ম্ল", keys: "mgV", sutonny: "¤ø" },

    // ল ligatures
    LigatureInfo { name: "ল্ক", unicode: "ল্ক", keys: "Vgj", sutonny: "é" },
    LigatureInfo { name: "ল্গ", unicode: "ল্গ", keys: "Vgo", sutonny: "ê" },
    LigatureInfo { name: "ল্ট", unicode: "ল্ট", keys: "Vgt", sutonny: "V‡U" },
    LigatureInfo { name: "ল্ড", unicode: "ল্ড", keys: "Vge", sutonny: "ì" },
    LigatureInfo { name: "ল্প", unicode: "ল্প", keys: "Vgr", sutonny: "í" },
    LigatureInfo { name: "ল্ফ", unicode: "ল্ফ", keys: "VgR", sutonny: "î" },
    LigatureInfo { name: "ল্ব", unicode: "ল্ব", keys: "Vgh", sutonny: "j¦" },
    LigatureInfo { name: "ল্ম", unicode: "ল্ম", keys: "Vgm", sutonny: "j¥" },
    LigatureInfo { name: "ল্ল", unicode: "ল্ল", keys: "VgV", sutonny: "jø" },

    // শ ligatures
    LigatureInfo { name: "শ্চ", unicode: "শ্চ", keys: "Mgy", sutonny: "ð" },
    LigatureInfo { name: "শ্ছ", unicode: "শ্ছ", keys: "MgY", sutonny: "ñ" },
    LigatureInfo { name: "শ্ন", unicode: "শ্ন", keys: "Mgb", sutonny: "kœ" },
    LigatureInfo { name: "শ্ব", unicode: "শ্ব", keys: "Mgh", sutonny: "k¦" },
    LigatureInfo { name: "শ্ম", unicode: "শ্ম", keys: "Mgm", sutonny: "k¥" },
    LigatureInfo { name: "শ্র", unicode: "শ্র", keys: "Mz",  sutonny: "kÖ" },
    LigatureInfo { name: "শ্ল", unicode: "শ্ল", keys: "MgV", sutonny: "kø" },

    // ষ ligatures
    LigatureInfo { name: "ষ্ক", unicode: "ষ্ক", keys: "Ngj", sutonny: "®‹" },
    LigatureInfo { name: "ষ্ট", unicode: "ষ্ট", keys: "Ngt", sutonny: "ó" },
    LigatureInfo { name: "ষ্ঠ", unicode: "ষ্ঠ", keys: "NgT", sutonny: "ô" },
    LigatureInfo { name: "ষ্ণ", unicode: "ষ্ণ", keys: "NgB", sutonny: "ò" },
    LigatureInfo { name: "ষ্প", unicode: "ষ্প", keys: "Ngr", sutonny: "®ú" },
    LigatureInfo { name: "ষ্ফ", unicode: "ষ্ফ", keys: "NgR", sutonny: "õ" },
    LigatureInfo { name: "ষ্ব", unicode: "ষ্ব", keys: "Ngh", sutonny: "õ" },
    LigatureInfo { name: "ষ্ম", unicode: "ষ্ম", keys: "Ngm", sutonny: "®§" },

    // স ligatures
    LigatureInfo { name: "স্ক", unicode: "স্ক", keys: "ngj", sutonny: "¯‹" },
    LigatureInfo { name: "স্খ", unicode: "স্খ", keys: "ngJ", sutonny: "ö" },
    LigatureInfo { name: "স্ট", unicode: "স্ট", keys: "ngt", sutonny: "÷" },
    LigatureInfo { name: "স্ত", unicode: "স্ত", keys: "ngk", sutonny: "¯Í" },
    LigatureInfo { name: "স্ত্ব", unicode: "স্ত্ব", keys: "ngkgh", sutonny: "¯Í¡" },
    LigatureInfo { name: "স্থ", unicode: "স্থ", keys: "ngK", sutonny: "¯’" },
    LigatureInfo { name: "স্ন", unicode: "স্ন", keys: "ngb", sutonny: "mœ" },
    LigatureInfo { name: "স্প", unicode: "স্প", keys: "ngr", sutonny: "¯ú" },
    LigatureInfo { name: "স্ফ", unicode: "স্ফ", keys: "ngR", sutonny: "ù" },
    LigatureInfo { name: "স্ব", unicode: "স্ব", keys: "ngh", sutonny: "¯^" },
    LigatureInfo { name: "স্ম", unicode: "স্ম", keys: "ngm", sutonny: "¯§" },
    LigatureInfo { name: "স্ল", unicode: "স্ল", keys: "ngV", sutonny: "¯ø" },
    LigatureInfo { name: "স্ত্র", unicode: "স্ত্র", keys: "ngkz", sutonny: "¯¿" },

    // হ ligatures
    LigatureInfo { name: "হ্ণ", unicode: "হ্ণ", keys: "igB", sutonny: "nè" },
    LigatureInfo { name: "হ্ন", unicode: "হ্ন", keys: "igb", sutonny: "ý" },
    LigatureInfo { name: "হ্ব", unicode: "হ্ব", keys: "igh", sutonny: "nŸ" },
    LigatureInfo { name: "হ্ম", unicode: "হ্ম", keys: "igm", sutonny: "þ" },
    LigatureInfo { name: "হ্ল", unicode: "হ্ল", keys: "igV", sutonny: "n¬" },
    LigatureInfo { name: "হ্র", unicode: "হ্র", keys: "iz",  sutonny: "n«" },
];
