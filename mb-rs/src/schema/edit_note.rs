#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditNote {
  pub id: i32,
  pub editor: i32,
  pub edit: i32,
  pub text: String,
  pub post_time: Option<chrono::DateTime<chrono::Utc>>,
}
