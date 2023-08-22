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
use embed::encoder;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params_from_iter;
use serde_json::json;

#[get("/")]
async fn hello(pool: web::Data<Pool<SqliteConnectionManager>>) -> impl Responder {
    let conn = pool.get().unwrap();

    let count_functions: u64 = conn
        .prepare_cached("SELECT COUNT(*) FROM functions;")
        .unwrap()
        .query_row([], |row| row.get(0))
        .unwrap();

    let count_analyses: u64 = conn
        .prepare_cached("SELECT COUNT(*) FROM function_analyses;")
        .unwrap()
        .query_row([], |row| row.get(0))
        .unwrap();

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
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    let q = path.to_string();
    let embeddings = enc.encode(&q);
    let doc_ids = vecdb.send(vector_db::Query(embeddings)).await.unwrap();
    let mut docs: Vec<String> = Vec::new();
    for doc in conn
        .prepare_cached(&format!(
            "SELECT id, name, signature FROM functions WHERE id IN ({});",
            repeat_vars(doc_ids.len())
        ))
        .unwrap()
        .query_map(params_from_iter(doc_ids.iter()), |row| {
            row.get::<usize, String>(2)
        })
        .unwrap()
    {
        docs.push(doc.unwrap());
    }

    HttpResponse::Ok().json(json!({"result": doc_ids, "test": docs}))
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
            .service(hello)
            .service(search)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    println!("Server Terminated. Byebye :-)");
    result
}

fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}
