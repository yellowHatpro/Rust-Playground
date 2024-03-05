#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct MediumAttributeTypeAllowedFormat {
  pub medium_format: i32,
  pub medium_attribute_type: i32,
}
