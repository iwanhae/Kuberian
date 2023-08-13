use super::model;

use anyhow::Error as E;
use candle_core::{Result, Tensor};
use std::time::Instant;
use tokenizers::Tokenizer;

pub struct Encoder {
    model: model::BertModel,
    tokenizer: Tokenizer,
}

impl Encoder {
    pub fn new(model: model::BertModel, tokenizer: Tokenizer) -> Self {
        let tokenizer = tokenizer.clone();
        Encoder { model, tokenizer }
    }
    pub fn encode(&self, prompt: &str) -> Result<Tensor> {
        let start = Instant::now();
        let tokens = self
            .tokenizer
            .encode(prompt, true)
            .map_err(E::msg)
            .unwrap()
            .get_ids()
            .to_vec();
        let token_ids = Tensor::new(&tokens[..], &self.model.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        let embeddings = self.model.forward(&token_ids, &token_type_ids)?;
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;

        // mean pooling
        assert_eq!(embeddings.shape().dims(), [1, 128, 384]);
        let embeddings = embeddings.sum(1)? / (n_tokens as f64);

        dbg!(start.elapsed());
        embeddings?.get(0)
    }
}
