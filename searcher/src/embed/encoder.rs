use super::model;

use anyhow::Error as E;
use candle::Tensor;
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
    pub fn encode(&self, prompt: &str) -> Vec<f32> {
        let start = Instant::now();
        let tokens = self
            .tokenizer
            .clone()
            .with_padding(None)
            .encode(prompt, true)
            .map_err(E::msg)
            .unwrap()
            .get_ids()
            .to_vec();
        let token_ids = Tensor::new(&tokens[..], &self.model.device)
            .unwrap()
            .unsqueeze(0)
            .unwrap();
        let token_type_ids = token_ids.zeros_like().unwrap();
        let embeddings = self.model.forward(&token_ids, &token_type_ids).unwrap();
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3().unwrap();

        // mean pooling
        let embeddings = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();
        debug!("{:?} {}", start.elapsed(), prompt);
        let tensor = normalize_l2(&embeddings).get(0).unwrap();

        let mut result: Vec<f32> = Vec::new();
        for i in 0..384 {
            result.push(tensor.get(i).unwrap().to_scalar::<f32>().unwrap())
        }
        result
    }
}

pub fn normalize_l2(v: &Tensor) -> Tensor {
    v.broadcast_div(&v.sqr().unwrap().sum_keepdim(1).unwrap().sqrt().unwrap())
        .unwrap()
}
