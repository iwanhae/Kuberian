use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseHello {
    pub total: u64,
    pub analyzed: u64,
    pub samples: Vec<String>,
}

#[derive(Serialize)]
pub struct ResponseSearch {
    pub query: String,
    pub docs: Vec<DocumentSummary>,
    pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub id: u64,
    pub score: f32,
}

#[derive(Serialize)]
pub struct DocumentSummary {
    pub id: u64,
    pub name: String,
    pub signature: String,
    pub file: String,
    pub line: LineInfo,
    pub summary: Option<String>,
}

#[derive(Serialize)]
pub struct LineInfo {
    pub start: i32,
    pub end: i32,
}
