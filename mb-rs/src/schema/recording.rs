#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Recording {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub artist_credit: i32,
  pub length: Option<i32>,
  pub comment: String,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub video: bool,
}
