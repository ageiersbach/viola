use super::utils::CharType;
use super::utils::PunctuationType;

pub struct Instrument {
    contents: String,
}

impl Instrument {
    pub fn new(s: String) -> Instrument {
        Instrument {
            contents: s,
        }
    }
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
            match CharType::from_char(c) {
                CharType::Space => {
                    words.push(s.clone());
                    s.clear();
                },
                CharType::Alphanumeric |
                CharType::Punctuation(PunctuationType::Apostrophe) |
                CharType::Punctuation(PunctuationType::Symbol) |
                CharType::Other => {
                    s.push(c);
                }
                CharType::Punctuation(_) => {},
            }
        }
        words.push(s);
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_contents() {
        let s = String::from("Hello world");
        assert_eq!(Instrument::new(s).contents, "Hello world");
    }

    #[test]
    fn length_gets_content_length() {
        let s = String::from("Hello world");
        assert_eq!(Instrument::new(s).length(), 11);
    }

    #[test]
    fn words_gets_vector_of_strings_from_content() {
        let s = String::from("Hello world");
        let s1 = String::from("Hello");
        let s2 = String::from("world");
        assert_eq!(Instrument::new(s).words(), vec![s1, s2]);

    }

    #[test]
    fn words_ignores_punctuation() {
        let s = String::from("hello, my dear. when's dinner? I'd like: apples, taters, and $100!");
        let v: Vec<String> = [
            String::from("hello"),
            String::from("my"),
            String::from("dear"),
            String::from("when's"),
            String::from("dinner"),
            String::from("I'd"),
            String::from("like"),
            String::from("apples"),
            String::from("taters"),
            String::from("and"),
            String::from("$100"),
        ].to_vec();
        let instrument = Instrument::new(s);
        assert_eq!(instrument.words(), v);
        assert_eq!(instrument.words().len(), 11);
    }
}
