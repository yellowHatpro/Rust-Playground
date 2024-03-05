#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAttributeTypeAllowedValue;

pub struct AreaAttributeTypeAllowedValueSet;

impl AreaAttributeTypeAllowedValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAttributeTypeAllowedValue>> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaAttributeTypeAllowedValue> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaAttributeTypeAllowedValue>> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaAttributeTypeAllowedValue>> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_attribute_type_id_where_area_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, area_attribute_type_id: i32) -> Result<Vec<AreaAttributeTypeAllowedValue>> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE area_attribute_type = $1"#)
            .bind(area_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_attribute_type_allowed_value_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, area_attribute_type_allowed_value_id: i32) -> Result<Vec<AreaAttributeTypeAllowedValue>> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(area_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute_type_allowed_value: AreaAttributeTypeAllowedValue) -> Result<AreaAttributeTypeAllowedValue> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"INSERT INTO "area_attribute_type_allowed_value" ("id", "area_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(area_attribute_type_allowed_value.id)
            .bind(area_attribute_type_allowed_value.gid)
            .bind(area_attribute_type_allowed_value.child_order)
            .bind(area_attribute_type_allowed_value.value)
            .bind(area_attribute_type_allowed_value.parent)
            .bind(area_attribute_type_allowed_value.description)
            .bind(area_attribute_type_allowed_value.area_attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute_type_allowed_value: AreaAttributeTypeAllowedValue) -> Result<AreaAttributeTypeAllowedValue> {
        query_as::<_, AreaAttributeTypeAllowedValue>(r#"UPDATE "area_attribute_type_allowed_value" SET "description" = $6, "child_order" = $5, "gid" = $7, "parent" = $4, "value" = $3, "area_attribute_type" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_attribute_type_allowed_value.gid)
            .bind(area_attribute_type_allowed_value.description)
            .bind(area_attribute_type_allowed_value.area_attribute_type)
            .bind(area_attribute_type_allowed_value.value)
            .bind(area_attribute_type_allowed_value.id)
            .bind(area_attribute_type_allowed_value.parent)
            .bind(area_attribute_type_allowed_value.child_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
