use std::{collections::HashSet, sync::LazyLock};

use serde::{Deserialize, Serialize};

use super::TokenFilter;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum StopwordsKind {
    Lucene,
    Nltk,
    Iso,
}

pub struct StopwordsTokenFilter {
    stopwords: &'static HashSet<&'static str>,
}

impl TokenFilter for StopwordsTokenFilter {
    fn apply(&self, token: String) -> Vec<String> {
        if self.stopwords.contains(token.as_str()) {
            vec![]
        } else {
            vec![token]
        }
    }
}

impl StopwordsTokenFilter {
    pub fn new(kind: StopwordsKind) -> Self {
        let words = match kind {
            StopwordsKind::Lucene => &*LICENE_STOPWORDS,
            StopwordsKind::Nltk => &*NLTK_STOPWORDS,
            StopwordsKind::Iso => &*ISO_STOPWORDS,
        };
        StopwordsTokenFilter { stopwords: words }
    }
}

macro_rules! STOPWORDS_DIR {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stopwords")
    };
}

static LUCENE_ENGLISH_STOPWORDS: &str = include_str!(concat!(STOPWORDS_DIR!(), "/lucene_english"));
static LICENE_STOPWORDS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| LUCENE_ENGLISH_STOPWORDS.lines().collect());

static NLTK_ENGLISH_STOPWORDS: &str = include_str!(concat!(STOPWORDS_DIR!(), "/nltk_english"));
static NLTK_STOPWORDS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| NLTK_ENGLISH_STOPWORDS.lines().collect());

static ISO_ENGLISH_STOPWORDS: &str = include_str!(concat!(STOPWORDS_DIR!(), "/iso_english"));
static ISO_STOPWORDS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| ISO_ENGLISH_STOPWORDS.lines().collect());
