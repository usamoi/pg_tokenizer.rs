mod jieba;
mod regex;
mod unicode_segmentation;

use std::sync::Arc;

use jieba::{create_jieba_pre_tokenizer, JiebaConfig};
use regex::RegexPreTokenizer;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentationPretokenizer;

pub trait PreTokenizer {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str>;
}
pub type PreTokenizerPtr = Arc<dyn PreTokenizer + Send + Sync>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum PreTokenizerConfig {
    Regex(String),
    UnicodeSegmentation,
    Jieba(JiebaConfig),
}

pub fn get_pre_tokenizer(config: PreTokenizerConfig) -> PreTokenizerPtr {
    match config {
        PreTokenizerConfig::Regex(pattern) => Arc::new(RegexPreTokenizer::new(&pattern)),
        PreTokenizerConfig::UnicodeSegmentation => Arc::new(UnicodeSegmentationPretokenizer),
        PreTokenizerConfig::Jieba(config) => create_jieba_pre_tokenizer(config),
    }
}
