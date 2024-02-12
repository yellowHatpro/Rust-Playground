#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct LinkType {
  pub id: i32,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub gid: uuid::Uuid,
  pub entity_type0: String,
  pub entity_type1: String,
  pub name: String,
  pub description: Option<String>,
  pub link_phrase: String,
  pub reverse_link_phrase: String,
  pub long_link_phrase: String,
  pub priority: i32,
  pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
  pub is_deprecated: bool,
  pub has_dates: bool,
  pub entity0_cardinality: i16,
  pub entity1_cardinality: i16,
}
