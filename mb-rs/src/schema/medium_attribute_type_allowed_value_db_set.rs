#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumAttributeTypeAllowedValue;

pub struct MediumAttributeTypeAllowedValueSet;

impl MediumAttributeTypeAllowedValueSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_medium_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_id: i32) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE medium_attribute_type = $1"#)
            .bind(medium_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_allowed_value_id: i32) -> Result<Vec<MediumAttributeTypeAllowedValue>> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(medium_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_value: MediumAttributeTypeAllowedValue) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"INSERT INTO "medium_attribute_type_allowed_value" ("id", "medium_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(medium_attribute_type_allowed_value.id)
            .bind(medium_attribute_type_allowed_value.medium_attribute_type)
            .bind(medium_attribute_type_allowed_value.value)
            .bind(medium_attribute_type_allowed_value.parent)
            .bind(medium_attribute_type_allowed_value.child_order)
            .bind(medium_attribute_type_allowed_value.description)
            .bind(medium_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute_type_allowed_value: MediumAttributeTypeAllowedValue) -> Result<MediumAttributeTypeAllowedValue> {
        query_as::<_, MediumAttributeTypeAllowedValue>(r#"UPDATE "medium_attribute_type_allowed_value" SET "medium_attribute_type" = $2, "value" = $3, "parent" = $4, "child_order" = $5, "description" = $6, "gid" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium_attribute_type_allowed_value.id)
            .bind(medium_attribute_type_allowed_value.medium_attribute_type)
            .bind(medium_attribute_type_allowed_value.value)
            .bind(medium_attribute_type_allowed_value.parent)
            .bind(medium_attribute_type_allowed_value.child_order)
            .bind(medium_attribute_type_allowed_value.description)
            .bind(medium_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
