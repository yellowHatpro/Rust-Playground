#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct PlaceRatingRaw {
  pub place: i32,
  pub editor: i32,
  pub rating: i16,
}
