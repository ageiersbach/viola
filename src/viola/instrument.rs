use super::utils::char_with_type;
use super::utils::CharType;

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
            match char_with_type(c) {
                CharType::Space => {
                    words.push(s.clone());
                    s.clear();
                },
                CharType::Punctuation => {},
                CharType::Other | CharType::Alphanumeric => {
                    s.push(c);
                }
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
        let s = String::from("hello, my! dear. when's dinner?");
        let v: Vec<String> = [
            String::from("hello"),
            String::from("my"),
            String::from("dear"),
            String::from("whens"),
            String::from("dinner"),
        ].to_vec();
        assert_eq!(Instrument::new(s).words(), v);
    }
}
