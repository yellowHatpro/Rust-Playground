#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct TagRelation {
  pub tag1: i32,
  pub tag2: i32,
  pub weight: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
