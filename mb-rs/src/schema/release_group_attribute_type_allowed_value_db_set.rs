#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAttributeTypeAllowedValue;

pub struct ReleaseGroupAttributeTypeAllowedValueSet;

impl ReleaseGroupAttributeTypeAllowedValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAttributeTypeAllowedValue> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_attribute_type_id_where_release_group_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, release_group_attribute_type_id: i32) -> Result<Vec<ReleaseGroupAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE release_group_attribute_type = $1"#)
            .bind(release_group_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_attribute_type_allowed_value_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, release_group_attribute_type_allowed_value_id: i32) -> Result<Vec<ReleaseGroupAttributeTypeAllowedValue>> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(release_group_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute_type_allowed_value: ReleaseGroupAttributeTypeAllowedValue) -> Result<ReleaseGroupAttributeTypeAllowedValue> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"INSERT INTO "release_group_attribute_type_allowed_value" ("id", "release_group_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(release_group_attribute_type_allowed_value.value)
            .bind(release_group_attribute_type_allowed_value.child_order)
            .bind(release_group_attribute_type_allowed_value.id)
            .bind(release_group_attribute_type_allowed_value.release_group_attribute_type)
            .bind(release_group_attribute_type_allowed_value.parent)
            .bind(release_group_attribute_type_allowed_value.description)
            .bind(release_group_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute_type_allowed_value: ReleaseGroupAttributeTypeAllowedValue) -> Result<ReleaseGroupAttributeTypeAllowedValue> {
        query_as::<_, ReleaseGroupAttributeTypeAllowedValue>(r#"UPDATE "release_group_attribute_type_allowed_value" SET "gid" = $7, "value" = $3, "parent" = $4, "child_order" = $5, "release_group_attribute_type" = $2, "description" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_attribute_type_allowed_value.description)
            .bind(release_group_attribute_type_allowed_value.release_group_attribute_type)
            .bind(release_group_attribute_type_allowed_value.gid)
            .bind(release_group_attribute_type_allowed_value.id)
            .bind(release_group_attribute_type_allowed_value.value)
            .bind(release_group_attribute_type_allowed_value.parent)
            .bind(release_group_attribute_type_allowed_value.child_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
