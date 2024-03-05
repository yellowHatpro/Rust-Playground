#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Instrument {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub Type: Option<i32>,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub comment: String,
  pub description: String,
}
