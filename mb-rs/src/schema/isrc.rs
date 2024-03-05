#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Isrc {
  pub id: i32,
  pub recording: i32,
  pub isrc: String,
  pub source: Option<i16>,
  pub edits_pending: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
