#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseFirstReleaseDate {
  pub release: i32,
  pub year: Option<i16>,
  pub month: Option<i16>,
  pub day: Option<i16>,
}
