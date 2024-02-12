#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Isrc {
  pub id: i32,
  pub recording: i32,
  pub isrc: char,
  pub source: Option<i16>,
  pub edits_pending: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
