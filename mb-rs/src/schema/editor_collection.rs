#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditorCollection {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub editor: i32,
  pub name: String,
  pub public: bool,
  pub description: String,
  pub Type: i32,
}
