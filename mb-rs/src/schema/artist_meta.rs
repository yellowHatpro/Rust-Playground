#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistMeta {
  pub id: i32,
  pub rating: Option<i16>,
  pub rating_count: Option<i32>,
}
