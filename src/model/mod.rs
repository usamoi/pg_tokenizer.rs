mod builtin;
mod custom;
mod huggingface;
mod lindera;

use std::sync::{Arc, LazyLock};

use builtin::{get_builtin_model, is_builtin_model};
use custom::{CustomModel, CustomModelConfig};
use dashmap::{DashMap, Entry};
use huggingface::{HuggingFaceConfig, HuggingFaceModel};
use lindera::{LinderaConfig, LinderaModel};
use serde::{Deserialize, Serialize};

use crate::utils::spi_get_one;

pub trait TokenizerModel {
    fn apply(&self, token: String) -> Vec<u32>;

    fn apply_batch(&self, tokens: Vec<String>) -> Vec<u32> {
        tokens
            .into_iter()
            .flat_map(|token| self.apply(token))
            .collect()
    }
}
pub type TokenizerModelPtr = Arc<dyn TokenizerModel + Send + Sync>;

impl TokenizerModel for tokenizers::Tokenizer {
    fn apply(&self, token: String) -> Vec<u32> {
        self.encode_fast(token, false).unwrap().get_ids().to_vec()
    }
}

impl TokenizerModel for tocken::tokenizer::Tokenizer {
    fn apply(&self, token: String) -> Vec<u32> {
        self.tokenize(&token)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
enum ModelConfig {
    Custom(CustomModelConfig),
    Lindera(LinderaConfig),
    HuggingFace(HuggingFaceConfig),
}

type ModelObjectPool = DashMap<String, TokenizerModelPtr>;
pub(super) static MODEL_OBJECT_POOL: LazyLock<ModelObjectPool> =
    LazyLock::new(ModelObjectPool::default);

pgrx::extension_sql!(
    r#"
CREATE TABLE tokenizer_catalog.model (
    name TEXT NOT NULL UNIQUE PRIMARY KEY,
    config TEXT NOT NULL
);
"#,
    name = "model_table"
);

pub fn get_model(name: &str) -> TokenizerModelPtr {
    validate_model_name(name).unwrap();
    if let Some(model) = MODEL_OBJECT_POOL.get(name) {
        return model.clone();
    }

    match MODEL_OBJECT_POOL.entry(name.to_string()) {
        Entry::Occupied(entry) => entry.get().clone(),
        Entry::Vacant(entry) => {
            if let Some(object) = get_builtin_model(name) {
                entry.insert(object.clone());
                return object;
            }

            if let Some(object) = get_model_from_database(name) {
                entry.insert(object.clone());
                return object;
            }

            panic!("Model not found: {}", name);
        }
    }
}

fn get_model_from_database(name: &str) -> Option<TokenizerModelPtr> {
    let config_bytes: &str = spi_get_one(
        "SELECT config FROM tokenizer_catalog.model WHERE name = $1",
        &[name.into()],
    )?;

    let config: ModelConfig = serde_json::from_str(config_bytes).unwrap();
    Some(build_model(name, &config))
}

fn build_model(name: &str, config: &ModelConfig) -> TokenizerModelPtr {
    match config {
        ModelConfig::Custom(config) => Arc::new(CustomModel::new(name, config)),
        ModelConfig::Lindera(config) => Arc::new(LinderaModel::new(config)),
        ModelConfig::HuggingFace(config) => Arc::new(HuggingFaceModel::new(name, config)),
    }
}

// 1. It only contains ascii letters, numbers, and underscores.
// 2. It starts with a letter.
// 3. Its length cannot exceed 20 characters.
pub fn validate_model_name(name: &str) -> Result<(), String> {
    let name_bytes = name.as_bytes();
    for &b in name_bytes {
        if !b.is_ascii_alphanumeric() && b != b'_' {
            return Err(format!("Invalid character: {}", b as char));
        }
    }
    if !(1..=20).contains(&name_bytes.len()) {
        return Err(format!("Name length must be between 1 and {}", 20));
    }
    if !name_bytes[0].is_ascii_alphabetic() {
        return Err("Name must start with a letter".to_string());
    }

    Ok(())
}

pub fn validate_new_model_name(name: &str) -> Result<(), String> {
    validate_model_name(name)?;
    if is_builtin_model(name) {
        return Err("The name is reserved, please choose another name".to_string());
    }

    Ok(())
}
