#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PlaceAnnotation {
  pub place: i32,
  pub annotation: i32,
}
