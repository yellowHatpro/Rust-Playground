#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct EditNoteChange {
  pub id: i32,
  pub status: Option<EditNoteStatus>,
  pub edit_note: i32,
  pub change_editor: i32,
  pub change_time: Option<chrono::DateTime<chrono::Utc>>,
  pub old_note: String,
  pub new_note: String,
  pub reason: String,
}
