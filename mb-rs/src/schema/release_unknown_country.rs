#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseUnknownCountry {
  pub release: i32,
  pub date_year: Option<i16>,
  pub date_month: Option<i16>,
  pub date_day: Option<i16>,
}
