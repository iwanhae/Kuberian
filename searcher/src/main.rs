#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[macro_use]
extern crate log;

mod args;
mod database;
mod embed;
mod server;
mod vector_db;

use actix_web::{middleware, web, App, HttpServer};
use args::Args;
use clap::Parser;
use embed::encoder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let args = Args::parse();

    // [BEGIN] INIT

    // MODEL
    let (model, tokenizer) = args.build_model_and_tokenizer().unwrap();
    let enc = encoder::Encoder::new(model, tokenizer);
    let app_data_encoder = web::Data::new(enc);
    info!("MODEL loaded");

    // DATABSE
    let database = database::new_connection_pool();
    let app_data_database = web::Data::new(database);
    info!("DATABASE loaded");

    // VECTOR DATABASE
    let vector = vector_db::start();
    let app_data_vector = web::Data::new(vector);
    info!("VECTOR DATABASE loaded");

    // [END] INIT
    args.terminate_if_ci();

    info!("Listen on 0.0.0.0:8080");

    let result = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(app_data_encoder.clone())
            .app_data(app_data_database.clone())
            .app_data(app_data_vector.clone())
            .service(server::handler::hello)
            .service(server::handler::search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    info!("Server Terminated. Byebye :-)");
    result
}
