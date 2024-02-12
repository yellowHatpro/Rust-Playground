#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Iswc {
  pub id: i32,
  pub work: i32,
  pub iswc: Option<char>,
  pub source: Option<i16>,
  pub edits_pending: i32,
  pub created: chrono::DateTime<chrono::Utc>,
}
