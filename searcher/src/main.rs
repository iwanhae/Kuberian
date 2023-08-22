#[cfg(feature = "mkl")]
extern crate intel_mkl_src;
mod args;
mod database;
mod embed;
mod vector_db;

use actix::Addr;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use args::Args;
use clap::Parser;
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use embed::encoder;
use serde_json::json;

use database::schema::function_analyses::dsl::*;
use database::schema::functions::dsl::*;

use crate::database::models::FunctionMeta;

#[get("/")]
async fn hello(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    let count_functions = functions
        .count()
        .get_result::<i64>(conn)
        .expect("can not get functions stats");

    let count_analyses = function_analyses
        .count()
        .get_result::<i64>(conn)
        .expect("can not get function_analyses stats");

    HttpResponse::Ok().json(json!({
        "total": count_functions,
        "analyzed": count_analyses
    }))
}

#[get("/q/{query}")]
async fn search(
    path: web::Path<String>,
    enc: web::Data<encoder::Encoder>,
    vecdb: web::Data<Addr<vector_db::VectorDB>>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    let q = path.to_string();
    let embeddings = enc.encode(&q);
    let doc_ids = vecdb.send(vector_db::Query(embeddings)).await.unwrap();

    let results: Vec<FunctionMeta> = functions
        .filter(database::schema::functions::dsl::id.eq_any(doc_ids.clone()))
        .select(database::models::FunctionMeta::as_select())
        .load(conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(json!({"result": doc_ids, "r": results}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // [BEGIN] INIT

    // MODEL
    let (model, tokenizer) = args.build_model_and_tokenizer().unwrap();
    let enc = encoder::Encoder::new(model, tokenizer);
    let app_data_encoder = web::Data::new(enc);

    // DATABSE
    let database = database::establish_connection();
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
            .service(hello)
            .service(search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    println!("Server Terminated. Byebye :-)");
    result
}
