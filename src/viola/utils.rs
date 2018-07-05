use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CharType {
    Space,
    Punctuation(PunctuationType),
    Alphanumeric,
    Other,
}

impl CharType {
    pub fn from_char(c: char) -> CharType {
        let c_int = c as u32;
        match c_int {
            0..=32 => CharType::Space, // includes space, tab, carriage returns but not DEL (127)
            33..=47 | 58..=64 | 91..=96 | 123..=126 => {
                CharType::Punctuation(PunctuationType::from_int(c_int))
            },
            48..=57 => CharType::Alphanumeric, // 0..9
            65..=90 => CharType::Alphanumeric, // 'A' .. 'Z'
            97..=122 => CharType::Alphanumeric, // 'a' .. 'z'
            _ => CharType::Other // emojis and other non-latin chars
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PunctuationType {
    Enclosure,
    Apostrophe,
    Break,
    Symbol,
}

impl PunctuationType {
    fn from_int(i: u32) -> PunctuationType {
        match i {
            33 | 44 | 46 | 58 | 59 | 63 => PunctuationType::Break, // ! , - . : ; ?
            34 | 40 | 41 | 91 | 93 | 123 | 125 => PunctuationType::Enclosure, // " () [] {}
            39 => PunctuationType::Apostrophe, // '
            _ => PunctuationType::Symbol, // # $ % & * + < = > @ ^ _ ` | ~
        }
    }
}

impl fmt::Display for CharType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_type = match *self {
            CharType::Space => "CharType::Space",
            CharType::Punctuation(_) => "CharType::Punctuation",
            CharType::Alphanumeric => "CharType::Punctuation",
            CharType::Other => "CharType::Other",
        };
        write!(f, "thing {}", char_type)
    }
}

impl fmt::Display for PunctuationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p_type = match *self {
            PunctuationType::Enclosure => "PunctuationType::Enclosure",
            PunctuationType::Apostrophe => "PunctuationType::Apostrophe",
            PunctuationType::Break => "PunctuationType::Break",
            PunctuationType::Symbol => "PunctuationType::Symbol",
        };
        write!(f, "thing {}", p_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_correct_type_for_alphanumeric_chars() {
        let alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let v: Vec<char> = alphanumeric.chars().collect();
        for i in &v {
            assert_eq!(CharType::from_char(*i), CharType::Alphanumeric);
        }
    }

    #[test]
    fn it_gets_correct_type_for_break_punctuation() {
        let punctuation = ":;,.!?"; // ! ,  . : ; ?
        let v: Vec<char> = punctuation.chars().collect();
        for i in &v {
            assert_eq!(CharType::from_char(*i), CharType::Punctuation(PunctuationType::Break));
        }
    }

    #[test]
    fn it_gets_correct_type_for_whitespace() {
        let spaces = " \t\r\n";
        let v: Vec<char> = spaces.chars().collect();
        for i in &v {
            assert_eq!(CharType::from_char(*i), CharType::Space);
        }
    }
}
