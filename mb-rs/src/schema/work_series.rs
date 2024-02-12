#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct WorkSeries {
  pub work: Option<i32>,
  pub series: Option<i32>,
  pub relationship: Option<i32>,
  pub link_order: Option<i32>,
  pub link: Option<i32>,
  pub text_value: Option<String>,
}
