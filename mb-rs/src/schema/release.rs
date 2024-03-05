#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Release {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub artist_credit: i32,
  pub release_group: i32,
  pub status: Option<i32>,
  pub packaging: Option<i32>,
  pub language: Option<i32>,
  pub script: Option<i32>,
  pub barcode: Option<String>,
  pub comment: String,
  pub edits_pending: i32,
  pub quality: i16,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
