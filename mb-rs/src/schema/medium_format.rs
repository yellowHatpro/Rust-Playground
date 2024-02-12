#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MediumFormat {
  pub id: i32,
  pub name: String,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub year: Option<i16>,
  pub has_discids: bool,
  pub description: Option<String>,
  pub gid: uuid::Uuid,
}
