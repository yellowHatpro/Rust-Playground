#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct MediumAttributeTypeAllowedValueAllowedFormat {
  pub medium_format: i32,
  pub medium_attribute_type_allowed_value: i32,
}
