#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct EventAttribute {
  pub id: i32,
  pub event: i32,
  pub event_attribute_type: i32,
  pub event_attribute_type_allowed_value: Option<i32>,
  pub event_attribute_text: Option<String>,
}
