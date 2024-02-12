#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct PlaceTagRaw {
  pub place: i32,
  pub editor: i32,
  pub tag: i32,
  pub is_upvote: bool,
}
