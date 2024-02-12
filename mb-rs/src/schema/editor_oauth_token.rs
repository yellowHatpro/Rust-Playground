#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EditorOauthToken {
  pub id: i32,
  pub editor: i32,
  pub application: i32,
  pub authorization_code: Option<String>,
  pub refresh_token: Option<String>,
  pub access_token: Option<String>,
  pub expire_time: chrono::DateTime<chrono::Utc>,
  pub scope: i32,
  pub granted: chrono::DateTime<chrono::Utc>,
  pub code_challenge: Option<String>,
  pub code_challenge_method: Option<oauth_code_challenge_method>,
}
