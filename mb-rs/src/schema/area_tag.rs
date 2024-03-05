#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AreaTag {
  pub area: i32,
  pub tag: i32,
  pub count: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
