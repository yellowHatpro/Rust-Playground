#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseLabel {
  pub id: i32,
  pub release: i32,
  pub label: Option<i32>,
  pub catalog_number: Option<String>,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
