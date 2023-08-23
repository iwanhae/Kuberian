use actix::Addr;
use actix_web::{get, web::Data, web::Path, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params_from_iter;
use serde_json::json;

use crate::{
    embed::encoder::Encoder,
    server::utils::repeat_vars,
    vector_db::{self, VectorDB},
};

#[get("/")]
async fn hello(pool: Data<Pool<SqliteConnectionManager>>) -> impl Responder {
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
    path: Path<String>,
    enc: Data<Encoder>,
    vecdb: Data<Addr<VectorDB>>,
    pool: Data<Pool<SqliteConnectionManager>>,
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
