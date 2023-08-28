use super::types;
use crate::{
    embed::encoder::Encoder,
    server::utils::repeat_vars,
    vector_db::{self, VectorDB},
};
use actix::Addr;
use actix_web::{
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params_from_iter;
use serde_json::json;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

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
        .prepare_cached(
            r"SELECT summary FROM function_analyses WHERE id 
            IN (SELECT id FROM function_analyses ORDER BY RANDOM() LIMIT 10);",
        )
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
    query_query: Path<String>,
    enc: Data<Encoder>,
    vecdb: Data<Addr<VectorDB>>,
    pool: Data<Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().unwrap();

    let q = query_query.to_string();
    let embeddings = enc.encode(&q);
    let doc_ids = vecdb.send(vector_db::Query(embeddings)).await.unwrap();
    let mut docs: Vec<types::Document> = Vec::new();
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
        Ok(types::Document {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            signature: row.get(2).unwrap(),
            file: row.get(3).unwrap(),
            line: types::LineInfo {
                start: row.get(4).unwrap(),
                end: row.get(5).unwrap(),
            },
            summary: row.get(6).unwrap(),
            extra: None,
        })
    })
    .expect("unexpected result from database")
    .for_each(|doc| {
        docs.push(doc.unwrap());
    });

    HttpResponse::Ok().json(types::ResponseSearch {
        query: q,
        results: Some(
            doc_ids
                .iter()
                .map(|x| types::SearchResult {
                    id: x.0,
                    score: x.1,
                })
                .collect(),
        ),
        docs,
    })
}

#[get("/dirs/kubernetes/kubernetes/v1.27.4")]
async fn list_dirs(pool: Data<Pool<SqliteConnectionManager>>) -> impl Responder {
    let conn = pool.get().unwrap();
    let mut docs: Vec<String> = Vec::new();

    conn.prepare_cached("SELECT DISTINCT file FROM functions;")
        .expect("fail to query on database")
        .query_map([], |row| row.get(0))
        .expect("unexpected result from database")
        .for_each(|doc| {
            docs.push(doc.unwrap());
        });

    HttpResponse::Ok().json(docs)
}

// TODO: reduce duplicated codes
#[get("/dirs/kubernetes/kubernetes/v1.27.4/{query:.*}")]
async fn list_dirs_docs(
    query_query: Path<String>,
    paging_query: Query<types::PagingQuery>,
    pool: Data<Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let paging = paging_query.defaults();
    let q = query_query.to_string();
    let mut docs: Vec<types::Document> = Vec::new();

    conn.prepare_cached(
        r#"
            SELECT src.id, name, signature, file, line_start, line_end, tgt.summary 
            FROM functions as src 
                LEFT JOIN function_analyses as tgt 
                ON src.id = tgt.function_id 
            WHERE src.file GLOB ? LIMIT ? OFFSET ?;;
            "#,
    )
    .expect("fail to query on database")
    .query_map(
        (format!("{}*", query_query), paging.limit, paging.offset),
        |row| {
            Ok(types::Document {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                signature: row.get(2).unwrap(),
                file: row.get(3).unwrap(),
                line: types::LineInfo {
                    start: row.get(4).unwrap(),
                    end: row.get(5).unwrap(),
                },
                summary: row.get(6).unwrap(),
                extra: None,
            })
        },
    )
    .expect("unexpected result from database")
    .for_each(|doc| {
        docs.push(doc.unwrap());
    });

    HttpResponse::Ok().json(types::ResponseSearch {
        query: q,
        results: None,
        docs,
    })
}

#[get("/docs/{id}")]
async fn get_docs(
    query_id: Path<u64>,
    pool: Data<Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    let mut docs: Vec<types::Document> = Vec::new();

    conn.prepare_cached(
        r#"
            SELECT src.id, name, signature, file, line_start, line_end, code, tgt.summary, tgt.background, tgt.analysis, tgt.purpose, tgt.comment, tgt.opinion
            FROM functions as src 
                LEFT JOIN function_analyses as tgt 
                ON src.id = tgt.function_id 
            WHERE src.id = ?;
            "#,
    )
    .expect("fail to query on database")
    .query_map([query_id.into_inner()], |row| {
        Ok(types::Document {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            signature: row.get(2).unwrap(),
            file: row.get(3).unwrap(),
            line: types::LineInfo {
                start: row.get(4).unwrap(),
                end: row.get(5).unwrap(),
            },
            summary: row.get(7).unwrap(),
            extra: Some(types::DocumentDetail{
                code: row.get(6).unwrap(),
                background: row.get(8).unwrap(),
                analysis: row.get(9).unwrap(),
                purpose: row.get(10).unwrap(),
                comment: row.get(11).unwrap(),
                opinion: row.get(12).unwrap(),
            }),
        })
    })
    .expect("unexpected result from database")
    .for_each(|doc| {
        docs.push(doc.unwrap());
    });

    if docs.len() == 0 {
        return HttpResponse::NotFound().json(json!({"msg": "Not Found :-("}));
    }

    HttpResponse::Ok().json(docs)
}
