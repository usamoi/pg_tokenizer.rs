use std::borrow::Cow;

use super::CharacterFilter;

pub struct ToLowercase;

impl CharacterFilter for ToLowercase {
    fn apply(&self, text: &mut Cow<str>) {
        *text = Cow::Owned(text.to_lowercase());
    }
}
