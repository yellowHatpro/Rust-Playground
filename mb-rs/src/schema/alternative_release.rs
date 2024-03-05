#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AlternativeRelease {
  pub id: i32,
  pub gid: uuid::Uuid,
  pub release: i32,
  pub name: Option<String>,
  pub artist_credit: Option<i32>,
  pub Type: i32,
  pub language: i32,
  pub script: i32,
  pub comment: String,
}
