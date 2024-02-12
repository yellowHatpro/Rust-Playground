#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct LinkAttributeType {
  pub id: i32,
  pub parent: Option<i32>,
  pub root: i32,
  pub child_order: i32,
  pub gid: uuid::Uuid,
  pub name: String,
  pub description: Option<String>,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}
