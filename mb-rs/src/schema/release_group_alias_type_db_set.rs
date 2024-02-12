#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAliasType;

pub struct ReleaseGroupAliasTypeSet;

impl ReleaseGroupAliasTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_group_alias_type_id<'e, E: PgExecutor<'e>>(executor: E, release_group_alias_type_id: i32) -> Result<Vec<ReleaseGroupAliasType>> {
        query_as::<_, ReleaseGroupAliasType>(r#"SELECT * FROM "musicbrainz"."release_group_alias_type" WHERE parent = $1"#)
            .bind(release_group_alias_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias_type: ReleaseGroupAliasType) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"INSERT INTO "release_group_alias_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_group_alias_type.id)
            .bind(release_group_alias_type.name)
            .bind(release_group_alias_type.parent)
            .bind(release_group_alias_type.child_order)
            .bind(release_group_alias_type.description)
            .bind(release_group_alias_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias_type: ReleaseGroupAliasType) -> Result<ReleaseGroupAliasType> {
        query_as::<_, ReleaseGroupAliasType>(r#"UPDATE "release_group_alias_type" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_alias_type.id)
            .bind(release_group_alias_type.name)
            .bind(release_group_alias_type.parent)
            .bind(release_group_alias_type.child_order)
            .bind(release_group_alias_type.description)
            .bind(release_group_alias_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_alias_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
