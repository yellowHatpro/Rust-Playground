#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct InstrumentAnnotation {
  pub instrument: i32,
  pub annotation: i32,
}
