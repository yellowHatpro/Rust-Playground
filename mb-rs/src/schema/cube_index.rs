#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct CubeIndex {
  pub medium: Option<i32>,
  pub toc: Option<Cube>,
}
