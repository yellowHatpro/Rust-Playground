#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseRaw {
  pub id: i32,
  pub title: String,
  pub artist: Option<String>,
  pub added: Option<chrono::DateTime<chrono::Utc>>,
  pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
  pub lookup_count: Option<i32>,
  pub modify_count: Option<i32>,
  pub source: Option<i32>,
  pub barcode: Option<String>,
  pub comment: String,
}
