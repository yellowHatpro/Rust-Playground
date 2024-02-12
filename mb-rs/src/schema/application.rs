#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Application {
  pub id: i32,
  pub owner: i32,
  pub name: String,
  pub oauth_id: String,
  pub oauth_secret: String,
  pub oauth_redirect_uri: Option<String>,
}
