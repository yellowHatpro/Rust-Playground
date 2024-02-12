#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Annotation {
  pub id: i32,
  pub editor: i32,
  pub text: Option<String>,
  pub changelog: Option<String>,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
