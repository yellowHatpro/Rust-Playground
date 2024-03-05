#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Cdtoc {
  pub id: i32,
  pub discid: String,
  pub freedb_id: String,
  pub track_count: i32,
  pub leadout_offset: i32,
  pub track_offset: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
