#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseGroupSecondaryTypeJoin {
  pub release_group: i32,
  pub secondary_type: i32,
  pub created: chrono::DateTime<chrono::Utc>,
}
