#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumAttributeTypeAllowedValueAllowedFormat;

pub struct MediumAttributeTypeAllowedValueAllowedFormatSet;

impl MediumAttributeTypeAllowedValueAllowedFormatSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_format_and_medium_attribute_type_allowed_value<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: i32, medium_attribute_type_allowed_value: i32) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "medium_format" = $1 AND "medium_attribute_type_allowed_value" = $2"#)
            .bind(medium_format)
            .bind(medium_attribute_type_allowed_value)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_format_and_medium_attribute_type_allowed_value_list<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format_list: Vec<i32>, medium_attribute_type_allowed_value_list: Vec<i32>) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "medium_format" = ANY($1) AND "medium_attribute_type_allowed_value" = ANY($2)"#)
            .bind(medium_format_list)
            .bind(medium_attribute_type_allowed_value_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_format_and_medium_attribute_type_allowed_value_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: i32, medium_attribute_type_allowed_value: i32) -> Result<Option<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "medium_format" = $1 AND "medium_attribute_type_allowed_value" = $2"#)
            .bind(medium_format)
            .bind(medium_attribute_type_allowed_value)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_medium_format_id<'e, E: PgExecutor<'e>>(executor: E, medium_format_id: i32) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE medium_format = $1"#)
            .bind(medium_format_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_allowed_value_id: i32) -> Result<Vec<MediumAttributeTypeAllowedValueAllowedFormat>> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE medium_attribute_type_allowed_value = $1"#)
            .bind(medium_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_value_allowed_format: MediumAttributeTypeAllowedValueAllowedFormat) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"INSERT INTO "medium_attribute_type_allowed_value_allowed_format" ("medium_format", "medium_attribute_type_allowed_value") VALUES ($1, $2) RETURNING *;"#)
            .bind(medium_attribute_type_allowed_value_allowed_format.medium_format)
            .bind(medium_attribute_type_allowed_value_allowed_format.medium_attribute_type_allowed_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_value_allowed_format: MediumAttributeTypeAllowedValueAllowedFormat) -> Result<MediumAttributeTypeAllowedValueAllowedFormat> {
        query_as::<_, MediumAttributeTypeAllowedValueAllowedFormat>(r#"UPDATE "medium_attribute_type_allowed_value_allowed_format" SET  WHERE "medium_format" = 1 AND "medium_attribute_type_allowed_value" = 2 RETURNING *;"#)
            .bind(medium_attribute_type_allowed_value_allowed_format.medium_format)
            .bind(medium_attribute_type_allowed_value_allowed_format.medium_attribute_type_allowed_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_attribute_type_allowed_value_allowed_format" WHERE "medium_format" = 1 AND "medium_attribute_type_allowed_value" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
