#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Medium {
  pub id: i32,
  pub release: i32,
  pub position: i32,
  pub format: Option<i32>,
  pub name: String,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub track_count: i32,
}
