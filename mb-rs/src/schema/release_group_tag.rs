#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseGroupTag {
  pub release_group: i32,
  pub tag: i32,
  pub count: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
