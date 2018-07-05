pub struct Instrument {
    contents: String,
}

pub enum CharType {
    Space,
    Punctuation,
    Other,
}

pub fn char_with_type(c: char) -> CharType {
    match c {
        ' ' => CharType::Space,
        ',' => CharType::Punctuation,
        '.' => CharType::Punctuation,
        ';' => CharType::Punctuation,
        '!' => CharType::Punctuation,
        '?' => CharType::Punctuation,
        _ => CharType::Other
    }
}

impl Instrument {
    pub fn length(&self) -> usize {
        self.contents.len()
    }
    pub fn chars(&self) -> Vec<char> {
        self.contents.chars().collect()
    }
    pub fn words(&self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut s: String = String::new();
        for c in self.chars() {
            match char_with_type(c) {
                CharType::Space => {
                    words.push(s.clone());
                    s.clear();
                },
                CharType::Punctuation => {},
                CharType::Other => {
                    s.push(c);
                }
            }
        }
        words.push(s);
        words
    }
}

pub mod viola {
    use Instrument;

    pub fn instrument(s: String) -> Instrument {
        Instrument {
            contents: s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_length_of_string() {
        let s = String::from("Hello world");
        assert_eq!(viola::instrument(s).length(), 11);
    }

    #[test]
    fn it_sets_contents_on_instrument() {
        let s = String::from("Hello world");
        assert_eq!(viola::instrument(s).contents, "Hello world");
    }

    #[test]
    fn it_gets_chars_on_instrument() {
        let s = String::from("Hello world");
        let v: Vec<char> = s.chars().collect();
        assert_eq!(viola::instrument(s).chars(), v);
    }

    #[test]
    fn it_gets_words_on_instrument() {
        let s = String::from("Hello world");
        let s1 = String::from("Hello");
        let s2 = String::from("world");
        assert_eq!(viola::instrument(s).words(), vec![s1, s2]);

    }

    #[test]
    fn it_ignores_punctuation_in_words_on_instrument() {
        let s = String::from("hello, my! dear. when's dinner?");
        let v: Vec<String> = [
            String::from("hello"),
            String::from("my"),
            String::from("dear"),
            String::from("when's"),
            String::from("dinner"),
        ].to_vec();
        assert_eq!(viola::instrument(s).words(), v);
    }
}
