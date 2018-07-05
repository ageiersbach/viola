#[derive(Debug)]
#[derive(PartialEq)]
pub enum CharType {
    Space,
    Punctuation,
    Alphanumeric,
    Other,
}

use std::fmt;
impl fmt::Display for CharType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_type = match *self {
            CharType::Space => "CharType::Space",
            CharType::Punctuation => "CharType::Punctuation",
            CharType::Alphanumeric => "CharType::Punctuation",
            CharType::Other => "CharType::Other",
        };
        write!(f, "thing {}", char_type)
    }
}

pub fn char_with_type(c: char) -> CharType {
    let c_int = c as u32;
    match c_int {
        0..=32 => CharType::Space, // includes space, tab, carriage returns but not DEL (127)
        33..=47 | 58..=64 | 91..=96 | 123..=126 => CharType::Punctuation,
        48..=57 => CharType::Alphanumeric, // 0..9
        65..=90 => CharType::Alphanumeric, // 'A' .. 'Z'
        97..=122 => CharType::Alphanumeric, // 'a' .. 'z'
        _ => CharType::Other // emojis and other non-latin chars
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
            assert_eq!(char_with_type(*i), CharType::Alphanumeric);
        }
    }

    #[test]
    fn it_gets_correct_type_for_punctuation() {
        let punctuation = ";,.!-?:";
        let v: Vec<char> = punctuation.chars().collect();
        for i in &v {
            assert_eq!(char_with_type(*i), CharType::Punctuation);
        }
    }

    #[test]
    fn it_gets_correct_type_for_whitespace() {
        let spaces = " \t\r\n";
        let v: Vec<char> = spaces.chars().collect();
        for i in &v {
            assert_eq!(char_with_type(*i), CharType::Space);
        }
    }
}
