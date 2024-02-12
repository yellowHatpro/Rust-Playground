#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct UnreferencedRowLog {
  pub table_name: String,
  pub row_id: i32,
  pub inserted: Option<chrono::DateTime<chrono::Utc>>,
}
