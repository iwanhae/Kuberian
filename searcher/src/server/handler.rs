use super::types;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params_from_iter;

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

    let mut samples: Vec<String> = vec![];
    for row in conn
        .prepare_cached("SELECT summary FROM function_analyses ORDER BY RANDOM() LIMIT 10;")
        .unwrap()
        .query_map([], |row| row.get::<usize, String>(0))
        .unwrap()
        .map(|row| row.unwrap())
    {
        samples.push(row);
    }

    HttpResponse::Ok().json(types::ResponseHello {
        total: count_functions,
        analyzed: count_analyses,
        samples,
    })
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
    let mut docs: Vec<types::DocumentSummary> = Vec::new();
    conn.prepare_cached(&format!(
        r#"
            SELECT src.id, name, signature, file, line_start, line_end, tgt.summary 
            FROM functions as src 
                LEFT JOIN function_analyses as tgt 
                ON src.id = tgt.function_id 
            WHERE src.id IN ({});
            "#,
        repeat_vars(doc_ids.len())
    ))
    .expect("fail to query on database")
    .query_map(params_from_iter(doc_ids.iter().map(|x| x.0)), |row| {
        Ok(types::DocumentSummary {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            signature: row.get(2).unwrap(),
            file: row.get(3).unwrap(),
            line: types::LineInfo {
                start: row.get(4).unwrap(),
                end: row.get(5).unwrap(),
            },
            summary: row.get(6).unwrap(),
        })
    })
    .expect("unexpected result from database")
    .for_each(|doc| {
        docs.push(doc.unwrap());
    });

    HttpResponse::Ok().json(types::ResponseSearch {
        query: q,
        results: doc_ids
            .iter()
            .map(|x| types::SearchResult {
                id: x.0,
                score: x.1,
            })
            .collect(),
        docs,
    })
}
