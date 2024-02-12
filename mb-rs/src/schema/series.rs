#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Series {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub comment: String,
  pub Type: i32,
  pub ordering_type: i32,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
