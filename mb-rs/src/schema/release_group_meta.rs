#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseGroupMeta {
  pub id: i32,
  pub release_count: i32,
  pub first_release_date_year: Option<i16>,
  pub first_release_date_month: Option<i16>,
  pub first_release_date_day: Option<i16>,
  pub rating: Option<i16>,
  pub rating_count: Option<i32>,
}
