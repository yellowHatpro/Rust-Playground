#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct LinkAttribute {
  pub link: i32,
  pub attribute_type: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
