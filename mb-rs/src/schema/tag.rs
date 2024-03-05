#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Tag {
  pub id: i32,
  pub name: String,
  pub ref_count: i32,
}
