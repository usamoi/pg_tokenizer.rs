use std::sync::Arc;

use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use pgrx::IntoDatum;
use serde::{Deserialize, Serialize};

use super::{validate_new_model_name, ModelConfig, TokenizerModel, MODEL_OBJECT_POOL};

#[derive(Debug, Serialize, Deserialize)]

pub struct LinderaConfig {
    #[serde(flatten)]
    inner: TokenizerConfig,
}

pub struct LinderaModel {
    tokenizer: Tokenizer,
}

impl LinderaModel {
    pub fn new(config: &LinderaConfig) -> Self {
        let tokenizer = Tokenizer::from_config(&config.inner).unwrap();
        Self { tokenizer }
    }
}

impl TokenizerModel for LinderaModel {
    fn apply(&self, token: String) -> Vec<u32> {
        self.tokenizer
            .tokenize(&token)
            .unwrap()
            .into_iter()
            .map(|t| t.word_id.id)
            .filter(|id| *id != u32::MAX)
            .collect()
    }
}

#[pgrx::pg_extern(volatile, parallel_safe)]
fn create_lindera_model(name: &str, config: &str) {
    validate_new_model_name(name).unwrap();
    let config: LinderaConfig = toml::from_str(config).unwrap();

    let insert_model = r#"
        INSERT INTO tokenizer_catalog.model (name, config) VALUES ($1, $2)
        ON CONFLICT (name) DO NOTHING RETURNING 1
    "#;

    let lindera_model = LinderaModel::new(&config);
    let config_str = serde_json::to_string(&ModelConfig::Lindera(config)).unwrap();

    pgrx::Spi::connect(|mut client| {
        let tuptable = client
            .update(
                insert_model,
                Some(1),
                Some(vec![
                    (pgrx::PgBuiltInOids::TEXTOID.oid(), name.into_datum()),
                    (pgrx::PgBuiltInOids::TEXTOID.oid(), config_str.into_datum()),
                ]),
            )
            .unwrap();

        if tuptable.len() == 0 {
            panic!("Model already exists: {}", name);
        }

        if MODEL_OBJECT_POOL
            .insert(name.to_string(), Arc::new(lindera_model))
            .is_some()
        {
            panic!("Model already exists: {}", name);
        }
    });
}

#[pgrx::pg_extern(volatile, parallel_safe)]
fn drop_lindera_model(name: &str) {
    validate_new_model_name(name).unwrap();

    let delete_model = r#"
        DELETE FROM tokenizer_catalog.model WHERE name = $1 RETURNING 1
    "#;

    pgrx::Spi::connect(|mut client| {
        let tuptable = client
            .update(
                delete_model,
                Some(1),
                Some(vec![(
                    pgrx::PgBuiltInOids::TEXTOID.oid(),
                    name.into_datum(),
                )]),
            )
            .unwrap();

        if tuptable.len() == 0 {
            pgrx::warning!("Model not found: {}", name);
        }
    });

    MODEL_OBJECT_POOL.remove(name);
}
