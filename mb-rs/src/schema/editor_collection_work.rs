#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditorCollectionWork {
  pub collection: i32,
  pub work: i32,
  pub added: Option<chrono::DateTime<chrono::Utc>>,
  pub position: i32,
  pub comment: String,
}
