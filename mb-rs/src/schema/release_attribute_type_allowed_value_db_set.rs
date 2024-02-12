#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseAttributeTypeAllowedValue;

pub struct ReleaseAttributeTypeAllowedValueSet;

impl ReleaseAttributeTypeAllowedValueSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, release_attribute_type_id: i32) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE release_attribute_type = $1"#)
            .bind(release_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, release_attribute_type_allowed_value_id: i32) -> Result<Vec<ReleaseAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(release_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_attribute_type_allowed_value: ReleaseAttributeTypeAllowedValue) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"INSERT INTO "release_attribute_type_allowed_value" ("id", "release_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(release_attribute_type_allowed_value.id)
            .bind(release_attribute_type_allowed_value.release_attribute_type)
            .bind(release_attribute_type_allowed_value.value)
            .bind(release_attribute_type_allowed_value.parent)
            .bind(release_attribute_type_allowed_value.child_order)
            .bind(release_attribute_type_allowed_value.description)
            .bind(release_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_attribute_type_allowed_value: ReleaseAttributeTypeAllowedValue) -> Result<ReleaseAttributeTypeAllowedValue> {
        query_as::<_, ReleaseAttributeTypeAllowedValue>(r#"UPDATE "release_attribute_type_allowed_value" SET "release_attribute_type" = $2, "value" = $3, "parent" = $4, "child_order" = $5, "description" = $6, "gid" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_attribute_type_allowed_value.id)
            .bind(release_attribute_type_allowed_value.release_attribute_type)
            .bind(release_attribute_type_allowed_value.value)
            .bind(release_attribute_type_allowed_value.parent)
            .bind(release_attribute_type_allowed_value.child_order)
            .bind(release_attribute_type_allowed_value.description)
            .bind(release_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
