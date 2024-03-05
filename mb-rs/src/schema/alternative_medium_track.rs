#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AlternativeMediumTrack {
  pub alternative_medium: i32,
  pub track: i32,
  pub alternative_track: i32,
}
