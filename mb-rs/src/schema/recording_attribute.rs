#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct RecordingAttribute {
  pub id: i32,
  pub recording: i32,
  pub recording_attribute_type: i32,
  pub recording_attribute_type_allowed_value: Option<i32>,
  pub recording_attribute_text: Option<String>,
}
