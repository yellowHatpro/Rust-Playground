#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditArtist {
  pub edit: i32,
  pub artist: i32,
  pub status: i16,
}
