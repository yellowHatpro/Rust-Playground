#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditReleaseGroup {
  pub edit: i32,
  pub release_group: i32,
}
