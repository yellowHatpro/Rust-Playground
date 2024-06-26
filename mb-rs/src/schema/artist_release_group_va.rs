#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistReleaseGroupVa {
  pub is_track_artist: bool,
  pub artist: i32,
  pub unofficial: bool,
  pub primary_type: Option<i16>,
  pub secondary_types: Option<i16>,
  pub first_release_date: Option<i32>,
  pub sort_character: String,
  pub release_group: i32,
}
