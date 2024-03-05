#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistCreditName {
  pub artist_credit: i32,
  pub position: i16,
  pub artist: i32,
  pub name: String,
  pub join_phrase: String,
}
