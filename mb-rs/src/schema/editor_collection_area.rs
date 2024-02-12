#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorCollectionArea {
  pub collection: i32,
  pub area: i32,
  pub added: Option<chrono::DateTime<chrono::Utc>>,
  pub position: i32,
  pub comment: String,
}
