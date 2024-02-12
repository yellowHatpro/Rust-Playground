#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct RecordingFirstReleaseDate {
  pub recording: i32,
  pub year: Option<i16>,
  pub month: Option<i16>,
  pub day: Option<i16>,
}
