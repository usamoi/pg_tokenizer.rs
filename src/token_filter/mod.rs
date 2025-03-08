mod pg_dict;
mod skip_non_alphanumeric;
mod stemmer;
mod stopwords;

use std::sync::Arc;

use pg_dict::PgDictTokenFilter;
use serde::{Deserialize, Serialize};
use skip_non_alphanumeric::SkipNonAlphanumeric;
use stemmer::{StemmerKind, StemmerTokenFilter};
use stopwords::{StopwordsKind, StopwordsTokenFilter};

pub trait TokenFilter {
    fn apply(&self, token: String) -> Vec<String>;

    fn apply_batch(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            .flat_map(|token| self.apply(token))
            .collect()
    }
}
pub type TokenFilterPtr = Arc<dyn TokenFilter + Sync + Send>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum TokenFilterConfig {
    SkipNonAlphanumeric,
    Stemmer(StemmerKind),
    Stopwords(StopwordsKind),
    PgDict(String),
}

pub fn get_token_filter(config: TokenFilterConfig) -> TokenFilterPtr {
    match config {
        TokenFilterConfig::SkipNonAlphanumeric => Arc::new(SkipNonAlphanumeric),
        TokenFilterConfig::Stemmer(kind) => Arc::new(StemmerTokenFilter::new(kind)),
        TokenFilterConfig::Stopwords(kind) => Arc::new(StopwordsTokenFilter::new(kind)),
        TokenFilterConfig::PgDict(name) => Arc::new(PgDictTokenFilter::new(&name)),
    }
}
