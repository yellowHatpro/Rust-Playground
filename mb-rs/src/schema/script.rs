#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Script {
  pub id: i32,
  pub iso_code: String,
  pub iso_number: String,
  pub name: String,
  pub frequency: i16,
}
