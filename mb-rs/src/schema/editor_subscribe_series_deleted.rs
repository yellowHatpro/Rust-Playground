#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorSubscribeSeriesDeleted {
  pub editor: i32,
  pub gid: uuid::Uuid,
  pub deleted_by: i32,
}
