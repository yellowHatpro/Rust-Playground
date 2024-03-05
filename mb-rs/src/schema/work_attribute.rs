#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct WorkAttribute {
  pub id: i32,
  pub work: i32,
  pub work_attribute_type: i32,
  pub work_attribute_type_allowed_value: Option<i32>,
  pub work_attribute_text: Option<String>,
}
