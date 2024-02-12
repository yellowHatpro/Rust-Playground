#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorPreference {
  pub id: i32,
  pub editor: i32,
  pub name: String,
  pub value: String,
}
