#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Track {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub recording: i32,
  pub medium: i32,
  pub position: i32,
  pub number: String,
  pub name: String,
  pub artist_credit: i32,
  pub length: Option<i32>,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub is_data_track: bool,
}
