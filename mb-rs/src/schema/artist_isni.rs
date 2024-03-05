#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistIsni {
  pub artist: i32,
  pub isni: String,
  pub edits_pending: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
