use std::sync::Arc;

use super::TokenizerModelPtr;

macro_rules! MODEL_DIR {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/model")
    };
}

static BERT_BASE_UNCASED_BYTES: &[u8] =
    include_bytes!(concat!(MODEL_DIR!(), "/bert_base_uncased.json"));
static TOCKEN_STR: &str = include_str!(concat!(MODEL_DIR!(), "/wiki_tocken.json"));

pub fn is_builtin_model(name: &str) -> bool {
    matches!(name, "bert" | "tocken")
}

pub fn get_builtin_model(name: &str) -> Option<TokenizerModelPtr> {
    match name {
        "bert" => Some(Arc::new(
            tokenizers::Tokenizer::from_bytes(BERT_BASE_UNCASED_BYTES).unwrap(),
        )),
        "tocken" => Some(Arc::new(tocken::tokenizer::Tokenizer::loads(TOCKEN_STR))),
        _ => None,
    }
}
