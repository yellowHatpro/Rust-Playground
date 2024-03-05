#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditorSubscribeLabel {
  pub id: i32,
  pub editor: i32,
  pub label: i32,
  pub last_edit_sent: i32,
}
