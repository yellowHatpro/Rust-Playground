#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PlaceAttribute {
  pub id: i32,
  pub place: i32,
  pub place_attribute_type: i32,
  pub place_attribute_type_allowed_value: Option<i32>,
  pub place_attribute_text: Option<String>,
}
