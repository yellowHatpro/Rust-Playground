#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Script {
  pub id: i32,
  pub iso_code: char,
  pub iso_number: char,
  pub name: String,
  pub frequency: i16,
}
