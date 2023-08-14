#[cfg(feature = "mkl")]
extern crate intel_mkl_src;
mod args;
mod embed;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use args::Args;
use clap::Parser;
use embed::encoder;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(":-)")
}

#[get("/q/{query}")]
async fn search(path: web::Path<String>, enc: web::Data<encoder::Encoder>) -> impl Responder {
    let q = path.to_string();
    let vec = enc.encode(&q).unwrap().to_string();
    HttpResponse::Ok().body(vec)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let (model, tokenizer) = args.build_model_and_tokenizer().unwrap();
    let enc = encoder::Encoder::new(model, tokenizer);
    let mutexed_enc = web::Data::new(enc);

    args.terminate_if_ci();

    println!("Listen on 0.0.0.0:8080");
    let result = HttpServer::new(move || {
        App::new()
            .app_data(mutexed_enc.clone())
            .service(hello)
            .service(search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    println!("Server Terminated. Byebye :-)");
    result
}
