use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct PagingQuery {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

pub struct Paging {
    pub offset: u64,
    pub limit: u64,
}

impl PagingQuery {
    pub fn defaults(&self) -> Paging {
        Paging {
            limit: self.limit.unwrap_or(10),
            offset: self.offset.unwrap_or(0),
        }
    }
}

#[derive(Serialize)]
pub struct ResponseHello {
    pub total: u64,
    pub analyzed: u64,
    pub samples: Vec<String>,
}

#[derive(Serialize)]
pub struct ResponseSearch {
    pub query: String,
    pub docs: Vec<Document>,
    pub results: Option<Vec<SearchResult>>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub id: u64,
    pub score: f32,
}

#[derive(Serialize)]
pub struct Document {
    pub id: u64,
    pub name: String,
    pub signature: String,
    pub file: String,
    pub line: LineInfo,
    pub summary: Option<String>,

    pub extra: Option<DocumentDetail>,
}

#[derive(Serialize)]
pub struct DocumentDetail {
    pub code: String,
    pub background: Option<String>,
    pub analysis: Option<String>,
    pub purpose: Option<String>,
    pub comment: Option<String>,
    pub opinion: Option<String>,
}

#[derive(Serialize)]
pub struct LineInfo {
    pub start: i32,
    pub end: i32,
}
