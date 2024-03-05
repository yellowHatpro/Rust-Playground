#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct RecordingSeries {
  pub recording: Option<i32>,
  pub series: Option<i32>,
  pub relationship: Option<i32>,
  pub link_order: Option<i32>,
  pub link: Option<i32>,
  pub text_value: Option<String>,
}
