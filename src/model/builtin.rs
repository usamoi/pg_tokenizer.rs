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
static GEMMA2B_BYTES: &[u8] = include_bytes!(concat!(MODEL_DIR!(), "/gemma2b.json"));
static LLMLINGUA2_BYTES: &[u8] = include_bytes!(concat!(MODEL_DIR!(), "/llmlingua2.json"));

pub fn is_builtin_model(name: &str) -> bool {
    matches!(
        name,
        "bert_base_uncased" | "wiki_tocken" | "gemma2b" | "llmlingua2"
    )
}

pub fn get_builtin_model(name: &str) -> Option<TokenizerModelPtr> {
    match name {
        "bert_base_uncased" => Some(Arc::new(
            tokenizers::Tokenizer::from_bytes(BERT_BASE_UNCASED_BYTES).unwrap(),
        )),
        "wiki_tocken" => Some(Arc::new(tocken::tokenizer::Tokenizer::loads(TOCKEN_STR))),
        "gemma2b" => Some(Arc::new(
            tokenizers::Tokenizer::from_bytes(GEMMA2B_BYTES).unwrap(),
        )),
        "llmlingua2" => Some(Arc::new(
            tokenizers::Tokenizer::from_bytes(LLMLINGUA2_BYTES).unwrap(),
        )),
        _ => None,
    }
}
