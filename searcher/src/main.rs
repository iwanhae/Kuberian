mod args;
mod embed;

use anyhow::Result;
use args::Args;
use clap::Parser;
use embed::encoder;

fn main() -> Result<()> {
    let args = Args::parse();
    let (model, tokenizer) = args.build_model_and_tokenizer()?;
    let enc = encoder::Encoder::new(model, tokenizer);

    for _ in 1..10 {
        let ys = enc.encode("Hello World").unwrap();
        println!("{:?}", ys.shape())
    }
    Ok(())
}
