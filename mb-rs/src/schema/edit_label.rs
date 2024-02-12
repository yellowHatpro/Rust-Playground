#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditLabel {
  pub edit: i32,
  pub label: i32,
  pub status: i16,
}
