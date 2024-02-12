#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct LGenreInstrument {
  pub id: i32,
  pub link: i32,
  pub entity0: i32,
  pub entity1: i32,
  pub edits_pending: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub link_order: i32,
  pub entity0_credit: String,
  pub entity1_credit: String,
}
