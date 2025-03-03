mod skip_non_alphanumeric;
mod stemmer;
mod stopwords;

use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use skip_non_alphanumeric::SkipNonAlphanumeric;
use stemmer::{StemmerKind, StemmerTokenFilter};
use stopwords::{StopwordsKind, StopwordsTokenFilter};

pub trait TokenFilter {
    // return true if the token should be kept
    fn apply(&self, token: &mut Cow<str>) -> bool;
}
pub type TokenFilterPtr = Arc<dyn TokenFilter + Sync + Send>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum TokenFilterConfig {
    SkipNonAlphanumeric,
    Stemmer(StemmerKind),
    Stopwords(StopwordsKind),
}

pub fn get_token_filter(config: TokenFilterConfig) -> TokenFilterPtr {
    match config {
        TokenFilterConfig::SkipNonAlphanumeric => Arc::new(SkipNonAlphanumeric),
        TokenFilterConfig::Stemmer(kind) => Arc::new(StemmerTokenFilter::new(kind)),
        TokenFilterConfig::Stopwords(kind) => Arc::new(StopwordsTokenFilter::new(kind)),
    }
}
