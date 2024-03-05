#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct MediumCdtoc {
  pub id: i32,
  pub medium: i32,
  pub cdtoc: i32,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
