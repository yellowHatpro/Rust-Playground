#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct Edit {
  pub id: i32,
  pub editor: i32,
  pub Type: i16,
  pub status: i16,
  pub autoedit: i16,
  pub open_time: Option<chrono::DateTime<chrono::Utc>>,
  pub close_time: Option<chrono::DateTime<chrono::Utc>>,
  pub expire_time: chrono::DateTime<chrono::Utc>,
  pub language: Option<i32>,
  pub quality: i16,
}
