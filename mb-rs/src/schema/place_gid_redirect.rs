#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct PlaceGidRedirect {
  pub gid: uuid::Uuid,
  pub new_id: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
