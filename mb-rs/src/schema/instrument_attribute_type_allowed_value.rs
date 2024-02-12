#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct InstrumentAttributeTypeAllowedValue {
  pub id: i32,
  pub instrument_attribute_type: i32,
  pub value: Option<String>,
  pub parent: Option<i32>,
  pub child_order: i32,
  pub description: Option<String>,
  pub gid: uuid::Uuid,
}
