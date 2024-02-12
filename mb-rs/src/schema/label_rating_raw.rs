#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct LabelRatingRaw {
  pub label: i32,
  pub editor: i32,
  pub rating: i16,
}
