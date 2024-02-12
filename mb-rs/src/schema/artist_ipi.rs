#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ArtistIpi {
  pub artist: i32,
  pub ipi: char,
  pub edits_pending: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
