#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseEvent {
  pub release: Option<i32>,
  pub date_year: Option<i16>,
  pub date_month: Option<i16>,
  pub date_day: Option<i16>,
  pub country: Option<i32>,
}
