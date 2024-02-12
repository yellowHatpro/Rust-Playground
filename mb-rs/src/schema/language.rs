#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Language {
  pub id: i32,
  pub iso_code_2t: Option<char>,
  pub iso_code_2b: Option<char>,
  pub iso_code_1: Option<char>,
  pub name: String,
  pub frequency: i16,
  pub iso_code_3: Option<char>,
}
