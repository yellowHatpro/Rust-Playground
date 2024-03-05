#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct LinkTextAttributeType {
  pub attribute_type: i32,
}
