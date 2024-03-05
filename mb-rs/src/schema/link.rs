#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Link {
  pub id: i32,
  pub link_type: i32,
  pub begin_date_year: Option<i16>,
  pub begin_date_month: Option<i16>,
  pub begin_date_day: Option<i16>,
  pub end_date_year: Option<i16>,
  pub end_date_month: Option<i16>,
  pub end_date_day: Option<i16>,
  pub attribute_count: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
  pub ended: bool,
}
