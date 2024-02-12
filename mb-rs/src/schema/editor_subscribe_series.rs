#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorSubscribeSeries {
  pub id: i32,
  pub editor: i32,
  pub series: i32,
  pub last_edit_sent: i32,
}
