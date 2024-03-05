#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct LinkTypeAttributeType {
  pub link_type: i32,
  pub attribute_type: i32,
  pub min: Option<i16>,
  pub max: Option<i16>,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
