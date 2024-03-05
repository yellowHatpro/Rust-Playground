#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Area {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub Type: Option<i32>,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub begin_date_year: Option<i16>,
  pub begin_date_month: Option<i16>,
  pub begin_date_day: Option<i16>,
  pub end_date_year: Option<i16>,
  pub end_date_month: Option<i16>,
  pub end_date_day: Option<i16>,
  pub ended: bool,
  pub comment: String,
}
