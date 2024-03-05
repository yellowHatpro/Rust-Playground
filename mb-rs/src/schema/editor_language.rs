#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditorLanguage {
  pub editor: i32,
  pub language: i32,
  pub fluency: Fluency,
}
