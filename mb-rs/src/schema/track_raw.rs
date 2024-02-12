#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct TrackRaw {
  pub id: i32,
  pub release: i32,
  pub title: String,
  pub artist: Option<String>,
  pub sequence: i32,
}
