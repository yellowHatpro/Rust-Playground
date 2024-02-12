#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorCollectionSeries {
  pub collection: i32,
  pub series: i32,
  pub added: Option<chrono::DateTime<chrono::Utc>>,
  pub position: i32,
  pub comment: String,
}
