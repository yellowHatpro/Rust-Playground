#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Artist {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub sort_name: String,
  pub begin_date_year: Option<i16>,
  pub begin_date_month: Option<i16>,
  pub begin_date_day: Option<i16>,
  pub end_date_year: Option<i16>,
  pub end_date_month: Option<i16>,
  pub end_date_day: Option<i16>,
  pub Type: Option<i32>,
  pub area: Option<i32>,
  pub gender: Option<i32>,
  pub comment: String,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub ended: bool,
  pub begin_area: Option<i32>,
  pub end_area: Option<i32>,
}
