#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AlternativeMedium {
  pub id: i32,
  pub medium: i32,
  pub alternative_release: i32,
  pub name: Option<String>,
}
