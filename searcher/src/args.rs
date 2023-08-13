use crate::embed;

use anyhow::{anyhow, Error as E, Result};
use candle_nn::VarBuilder;
use clap::Parser;
use embed::model::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::Api, Cache, Repo, RepoType};
use tokenizers::Tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run offline (you must have the files already cached)
    #[arg(long)]
    offline: bool,

    /// The model to use, check out available models: https://huggingface.co/models?library=sentence-transformers&sort=trending
    #[arg(long)]
    model_id: Option<String>,

    #[arg(long)]
    revision: Option<String>,
}

impl Args {
    pub fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
        let device = candle_examples::device(true)?;
        let default_model = "sentence-transformers/all-MiniLM-L6-v2".to_string();

        // source: https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/discussions/21
        let default_revision = "refs/pr/21".to_string();
        let (model_id, revision) = match (self.model_id.to_owned(), self.revision.to_owned()) {
            (Some(model_id), Some(revision)) => (model_id, revision),
            (Some(model_id), None) => (model_id, "main".to_string()),
            (None, Some(revision)) => (default_model, revision),
            (None, None) => (default_model, default_revision),
        };

        let repo = Repo::with_revision(model_id, RepoType::Model, revision);
        let (config_filename, tokenizer_filename, weights_filename) = if self.offline {
            let cache = Cache::default();
            (
                cache
                    .get(&repo, "config.json")
                    .ok_or(anyhow!("Missing config file in cache"))?,
                cache
                    .get(&repo, "tokenizer.json")
                    .ok_or(anyhow!("Missing tokenizer file in cache"))?,
                cache
                    .get(&repo, "model.safetensors")
                    .ok_or(anyhow!("Missing weights file in cache"))?,
            )
        } else {
            let api = Api::new()?;
            let api = api.repo(repo);
            (
                api.get("config.json")?,
                api.get("tokenizer.json")?,
                api.get("model.safetensors")?,
            )
        };
        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

        let weights = unsafe { candle_core::safetensors::MmapedFile::new(weights_filename)? };
        let weights = weights.deserialize()?;
        let vb = VarBuilder::from_safetensors(vec![weights], DTYPE, &device);
        let model = BertModel::load(vb, &config)?;
        Ok((model, tokenizer))
    }
}
