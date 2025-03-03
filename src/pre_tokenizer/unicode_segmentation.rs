use unicode_segmentation::UnicodeSegmentation;

use super::PreTokenizer;

pub struct UnicodeSegmentationPretokenizer;

impl PreTokenizer for UnicodeSegmentationPretokenizer {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        text.unicode_words().map(|s| s).collect()
    }
}
