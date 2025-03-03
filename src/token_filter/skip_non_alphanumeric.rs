use std::borrow::Cow;

use super::TokenFilter;

pub struct SkipNonAlphanumeric;

impl TokenFilter for SkipNonAlphanumeric {
    fn apply(&self, token: &mut Cow<str>) -> bool {
        token.chars().any(char::is_alphanumeric)
    }
}
