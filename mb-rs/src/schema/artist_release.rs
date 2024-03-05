#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistRelease {
  pub is_track_artist: bool,
  pub artist: i32,
  pub first_release_date: Option<i32>,
  pub catalog_numbers: Option<String>,
  pub country_code: Option<String>,
  pub barcode: Option<i64>,
  pub sort_character: String,
  pub release: i32,
}
