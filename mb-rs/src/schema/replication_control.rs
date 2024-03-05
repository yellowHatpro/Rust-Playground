#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct ReplicationControl {
  pub id: i32,
  pub current_schema_sequence: i32,
  pub current_replication_sequence: Option<i32>,
  pub last_replication_date: Option<chrono::DateTime<chrono::Utc>>,
}
