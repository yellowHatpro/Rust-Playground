#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesAttributeTypeAllowedValue;

pub struct SeriesAttributeTypeAllowedValueSet;

impl SeriesAttributeTypeAllowedValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesAttributeTypeAllowedValue>> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesAttributeTypeAllowedValue> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesAttributeTypeAllowedValue>> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesAttributeTypeAllowedValue>> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_attribute_type_id_where_series_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, series_attribute_type_id: i32) -> Result<Vec<SeriesAttributeTypeAllowedValue>> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE series_attribute_type = $1"#)
            .bind(series_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_attribute_type_allowed_value_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, series_attribute_type_allowed_value_id: i32) -> Result<Vec<SeriesAttributeTypeAllowedValue>> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(series_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_attribute_type_allowed_value: SeriesAttributeTypeAllowedValue) -> Result<SeriesAttributeTypeAllowedValue> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"INSERT INTO "series_attribute_type_allowed_value" ("id", "series_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(series_attribute_type_allowed_value.child_order)
            .bind(series_attribute_type_allowed_value.value)
            .bind(series_attribute_type_allowed_value.series_attribute_type)
            .bind(series_attribute_type_allowed_value.id)
            .bind(series_attribute_type_allowed_value.parent)
            .bind(series_attribute_type_allowed_value.description)
            .bind(series_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_attribute_type_allowed_value: SeriesAttributeTypeAllowedValue) -> Result<SeriesAttributeTypeAllowedValue> {
        query_as::<_, SeriesAttributeTypeAllowedValue>(r#"UPDATE "series_attribute_type_allowed_value" SET "child_order" = $5, "gid" = $7, "description" = $6, "series_attribute_type" = $2, "value" = $3, "parent" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_attribute_type_allowed_value.value)
            .bind(series_attribute_type_allowed_value.id)
            .bind(series_attribute_type_allowed_value.series_attribute_type)
            .bind(series_attribute_type_allowed_value.parent)
            .bind(series_attribute_type_allowed_value.child_order)
            .bind(series_attribute_type_allowed_value.description)
            .bind(series_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
