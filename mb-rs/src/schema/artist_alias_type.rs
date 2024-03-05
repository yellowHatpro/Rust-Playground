#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistAliasType {
  pub id: i32,
  pub name: String,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub description: Option<String>,
  pub gid: uuid::Uuid,
}
