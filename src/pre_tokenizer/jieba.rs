use std::sync::{Arc, LazyLock};

use jieba_rs::Jieba;
use serde::{Deserialize, Serialize};

use super::{PreTokenizer, PreTokenizerPtr};

static JIEBA: LazyLock<Jieba> = LazyLock::new(|| Jieba::new());

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JiebaMode {
    Full,
    Percise,
    Search,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct JiebaConfig {
    mode: JiebaMode,
    enable_hmm: bool,
}

impl Default for JiebaConfig {
    fn default() -> Self {
        Self {
            mode: JiebaMode::Search,
            enable_hmm: true,
        }
    }
}

struct JiebaFull {
    enable_hmm: bool,
}

struct JiebaPercise;

struct JiebaSearch {
    enable_hmm: bool,
}

impl PreTokenizer for JiebaFull {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        JIEBA.cut(text, self.enable_hmm)
    }
}

impl PreTokenizer for JiebaPercise {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        JIEBA.cut_all(text)
    }
}

impl PreTokenizer for JiebaSearch {
    fn pre_tokenize<'a>(&self, text: &'a str) -> Vec<&'a str> {
        JIEBA.cut_for_search(text, self.enable_hmm)
    }
}

pub fn create_jieba_pre_tokenizer(config: JiebaConfig) -> PreTokenizerPtr {
    match config.mode {
        JiebaMode::Full => Arc::new(JiebaFull {
            enable_hmm: config.enable_hmm,
        }),
        JiebaMode::Percise => Arc::new(JiebaPercise),
        JiebaMode::Search => Arc::new(JiebaSearch {
            enable_hmm: config.enable_hmm,
        }),
    }
}
