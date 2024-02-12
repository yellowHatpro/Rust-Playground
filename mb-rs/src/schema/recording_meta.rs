#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct RecordingMeta {
  pub id: i32,
  pub rating: Option<i16>,
  pub rating_count: Option<i32>,
}
