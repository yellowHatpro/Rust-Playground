#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditNoteRecipient {
  pub recipient: i32,
  pub edit_note: i32,
}
