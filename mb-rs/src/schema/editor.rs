#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Editor {
  pub id: i32,
  pub name: String,
  pub privs: Option<i32>,
  pub email: Option<String>,
  pub website: Option<String>,
  pub bio: Option<String>,
  pub member_since: Option<chrono::DateTime<chrono::Utc>>,
  pub email_confirm_date: Option<chrono::DateTime<chrono::Utc>>,
  pub last_login_date: Option<chrono::DateTime<chrono::Utc>>,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub birth_date: Option<chrono::NaiveDate>,
  pub gender: Option<i32>,
  pub area: Option<i32>,
  pub password: String,
  pub ha1: char,
  pub deleted: bool,
}
