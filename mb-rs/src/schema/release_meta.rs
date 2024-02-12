#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct ReleaseMeta {
  pub id: i32,
  pub date_added: Option<chrono::DateTime<chrono::Utc>>,
  pub info_url: Option<String>,
  pub amazon_asin: Option<String>,
  pub cover_art_presence: cover_art_presence,
}
