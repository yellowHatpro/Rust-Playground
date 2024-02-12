#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct InstrumentGidRedirect {
  pub gid: uuid::Uuid,
  pub new_id: i32,
  pub created: Option<chrono::DateTime<chrono::Utc>>,
}
