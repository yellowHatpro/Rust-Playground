#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Iswc {
  pub id: i32,
  pub work: i32,
  pub iswc: Option<String>,
  pub source: Option<i16>,
  pub edits_pending: i32,
  pub created: chrono::DateTime<chrono::Utc>,
}
