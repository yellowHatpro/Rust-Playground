#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct DeletedEntity {
  pub gid: uuid::Uuid,
  pub data: sqlx::Json,
  pub deleted_at: chrono::DateTime<chrono::Utc>,
}
