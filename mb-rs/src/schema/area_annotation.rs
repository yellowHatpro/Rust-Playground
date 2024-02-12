#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AreaAnnotation {
  pub area: i32,
  pub annotation: i32,
}
