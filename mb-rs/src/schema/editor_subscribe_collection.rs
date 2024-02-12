#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorSubscribeCollection {
  pub id: i32,
  pub editor: i32,
  pub collection: i32,
  pub last_edit_sent: i32,
  pub available: bool,
  pub last_seen_name: Option<String>,
}
