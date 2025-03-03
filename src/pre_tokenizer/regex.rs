use super::PreTokenizer;

pub struct RegexPreTokenizer {
    pattern: regex::Regex,
}

impl RegexPreTokenizer {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: regex::Regex::new(pattern).unwrap(),
        }
    }
}

impl PreTokenizer for RegexPreTokenizer {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.pattern.find_iter(text).map(|m| m.as_str()).collect()
    }
}
