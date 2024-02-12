#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MediumIndex {
  pub medium: i32,
  pub toc: Option<Cube>,
}
