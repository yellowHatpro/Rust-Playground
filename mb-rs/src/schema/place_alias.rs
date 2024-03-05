#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct PlaceAlias {
  pub id: i32,
  pub place: i32,
  pub name: String,
  pub locale: Option<String>,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub Type: Option<i32>,
  pub sort_name: String,
  pub begin_date_year: Option<i16>,
  pub begin_date_month: Option<i16>,
  pub begin_date_day: Option<i16>,
  pub end_date_year: Option<i16>,
  pub end_date_month: Option<i16>,
  pub end_date_day: Option<i16>,
  pub primary_for_locale: bool,
  pub ended: bool,
}
