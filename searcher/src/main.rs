#[cfg(feature = "mkl")]
extern crate intel_mkl_src;
mod args;
mod database;
mod embed;
mod server;
mod vector_db;

use actix_web::{web, App, HttpServer};
use args::Args;
use clap::Parser;
use embed::encoder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // [BEGIN] INIT

    // MODEL
    let (model, tokenizer) = args.build_model_and_tokenizer().unwrap();
    let enc = encoder::Encoder::new(model, tokenizer);
    let app_data_encoder = web::Data::new(enc);

    // DATABSE
    let database = database::new_connection_pool();
    let app_data_database = web::Data::new(database);

    // VECTOR DATABASE
    let vector = vector_db::start();
    let app_data_vector = web::Data::new(vector);

    // [END] INIT
    args.terminate_if_ci();

    println!("Listen on 0.0.0.0:8080");
    let result = HttpServer::new(move || {
        App::new()
            .app_data(app_data_encoder.clone())
            .app_data(app_data_database.clone())
            .app_data(app_data_vector.clone())
            .service(server::handler::hello)
            .service(server::handler::search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    println!("Server Terminated. Byebye :-)");
    result
}
