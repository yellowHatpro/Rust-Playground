#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ArtistAttributeTypeAllowedValue {
  pub id: i32,
  pub artist_attribute_type: i32,
  pub value: Option<String>,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub description: Option<String>,
  pub gid: uuid::Uuid,
}
