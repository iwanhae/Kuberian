use super::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::function_analyses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct FunctionAnalyzed {
    pub function_id: i32,
    pub summary: String,
    pub background: Option<String>,
    pub analysis: Option<String>,
    pub purpose: Option<String>,
    pub comment: Option<String>,
    pub tldr: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::functions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct FunctionMeta {
    pub id: i32,
    pub name: String,
    pub signature: String,
    pub file: String,
    pub code: String,
    pub line_start: i32,
    pub line_end: i32,
}
