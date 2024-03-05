#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseCountry {
  pub release: i32,
  pub country: i32,
  pub date_year: Option<i16>,
  pub date_month: Option<i16>,
  pub date_day: Option<i16>,
}
