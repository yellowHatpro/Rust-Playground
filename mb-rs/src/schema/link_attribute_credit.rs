#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct LinkAttributeCredit {
  pub link: i32,
  pub attribute_type: i32,
  pub credited_as: String,
}
