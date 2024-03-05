#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AlternativeTrack {
  pub id: i32,
  pub name: Option<String>,
  pub artist_credit: Option<i32>,
  pub ref_count: i32,
}
