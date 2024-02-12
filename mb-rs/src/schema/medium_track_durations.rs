#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MediumTrackDurations {
  pub medium: Option<i32>,
  pub pregap_length: Option<i32>,
  pub cdtoc_track_lengths: Option<i32>,
  pub data_track_lengths: Option<i32>,
}
