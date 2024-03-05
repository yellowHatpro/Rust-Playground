#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use crate::schema::types::*;
#[derive(sqlx::FromRow, Debug)]
pub struct AutoeditorElectionVote {
  pub id: i32,
  pub autoeditor_election: i32,
  pub voter: i32,
  pub vote: i32,
  pub vote_time: chrono::DateTime<chrono::Utc>,
}
