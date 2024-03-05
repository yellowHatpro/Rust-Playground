#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistCredit {
  pub id: i32,
  pub name: String,
  pub artist_count: i16,
  pub ref_count: Option<i32>,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
  pub edits_pending: i32,
  pub gid: uuid::Uuid,
}
