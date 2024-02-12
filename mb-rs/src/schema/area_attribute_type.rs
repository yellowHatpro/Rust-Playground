#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AreaAttributeType {
  pub id: i32,
  pub name: String,
  pub comment: String,
  pub free_text: bool,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub description: Option<String>,
  pub gid: uuid::Uuid,
}
