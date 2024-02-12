#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAttributeType;

pub struct ReleaseGroupAttributeTypeSet;

impl ReleaseGroupAttributeTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_group_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, release_group_attribute_type_id: i32) -> Result<Vec<ReleaseGroupAttributeType>> {
        query_as::<_, ReleaseGroupAttributeType>(r#"SELECT * FROM "musicbrainz"."release_group_attribute_type" WHERE parent = $1"#)
            .bind(release_group_attribute_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute_type: ReleaseGroupAttributeType) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"INSERT INTO "release_group_attribute_type" ("id", "name", "comment", "free_text", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(release_group_attribute_type.id)
            .bind(release_group_attribute_type.name)
            .bind(release_group_attribute_type.comment)
            .bind(release_group_attribute_type.free_text)
            .bind(release_group_attribute_type.parent)
            .bind(release_group_attribute_type.child_order)
            .bind(release_group_attribute_type.description)
            .bind(release_group_attribute_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute_type: ReleaseGroupAttributeType) -> Result<ReleaseGroupAttributeType> {
        query_as::<_, ReleaseGroupAttributeType>(r#"UPDATE "release_group_attribute_type" SET "name" = $2, "comment" = $3, "free_text" = $4, "parent" = $5, "child_order" = $6, "description" = $7, "gid" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_attribute_type.id)
            .bind(release_group_attribute_type.name)
            .bind(release_group_attribute_type.comment)
            .bind(release_group_attribute_type.free_text)
            .bind(release_group_attribute_type.parent)
            .bind(release_group_attribute_type.child_order)
            .bind(release_group_attribute_type.description)
            .bind(release_group_attribute_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_attribute_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
