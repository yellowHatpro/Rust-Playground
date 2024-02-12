#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AutoeditorElection {
  pub id: i32,
  pub candidate: i32,
  pub proposer: i32,
  pub seconder_1: Option<i32>,
  pub seconder_2: Option<i32>,
  pub status: i32,
  pub yes_votes: i32,
  pub no_votes: i32,
  pub propose_time: chrono::DateTime<chrono::Utc>,
  pub open_time: Option<chrono::DateTime<chrono::Utc>>,
  pub close_time: Option<chrono::DateTime<chrono::Utc>>,
}
