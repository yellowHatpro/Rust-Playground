#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumAttributeTypeAllowedFormat;

pub struct MediumAttributeTypeAllowedFormatSet;

impl MediumAttributeTypeAllowedFormatSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumAttributeTypeAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_medium_format_and_medium_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: i32, medium_attribute_type: i32) -> Result<MediumAttributeTypeAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE "medium_format" = $1 AND "medium_attribute_type" = $2"#)
            .bind(medium_format)
            .bind(medium_attribute_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_medium_format_and_medium_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format_list: Vec<i32>, medium_attribute_type_list: Vec<i32>) -> Result<Vec<MediumAttributeTypeAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE "medium_format" = ANY($1) AND "medium_attribute_type" = ANY($2)"#)
            .bind(medium_format_list)
            .bind(medium_attribute_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_medium_format_and_medium_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: i32, medium_attribute_type: i32) -> Result<Option<MediumAttributeTypeAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE "medium_format" = $1 AND "medium_attribute_type" = $2"#)
            .bind(medium_format)
            .bind(medium_attribute_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_medium_format_id_where_medium_format_is<'e, E: PgExecutor<'e>>(executor: E, medium_format_id: i32) -> Result<Vec<MediumAttributeTypeAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE medium_format = $1"#)
            .bind(medium_format_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_attribute_type_id_where_medium_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_id: i32) -> Result<Vec<MediumAttributeTypeAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE medium_attribute_type = $1"#)
            .bind(medium_attribute_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_format: MediumAttributeTypeAllowedFormat) -> Result<MediumAttributeTypeAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"INSERT INTO "medium_attribute_type_allowed_format" ("medium_format", "medium_attribute_type") VALUES ($1, $2) RETURNING *;"#)
            .bind(medium_attribute_type_allowed_format.medium_attribute_type)
            .bind(medium_attribute_type_allowed_format.medium_format)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_format: MediumAttributeTypeAllowedFormat) -> Result<MediumAttributeTypeAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedFormat>(r#"UPDATE "medium_attribute_type_allowed_format" SET  WHERE "medium_format" = 1 AND "medium_attribute_type" = 2 RETURNING *;"#)
            .bind(medium_attribute_type_allowed_format.medium_format)
            .bind(medium_attribute_type_allowed_format.medium_attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_attribute_type_allowed_format" WHERE "medium_format" = 1 AND "medium_attribute_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
