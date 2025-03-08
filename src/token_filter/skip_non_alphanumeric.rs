use super::TokenFilter;

pub struct SkipNonAlphanumeric;

impl TokenFilter for SkipNonAlphanumeric {
    fn apply(&self, token: String) -> Vec<String> {
        if token.chars().any(char::is_alphanumeric) {
            vec![token]
        } else {
            vec![]
        }
    }
}
