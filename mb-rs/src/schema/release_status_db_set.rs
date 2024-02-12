#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseStatus;

pub struct ReleaseStatusSet;

impl ReleaseStatusSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_status_id<'e, E: PgExecutor<'e>>(executor: E, release_status_id: i32) -> Result<Vec<ReleaseStatus>> {
        query_as::<_, ReleaseStatus>(r#"SELECT * FROM "musicbrainz"."release_status" WHERE parent = $1"#)
            .bind(release_status_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_status: ReleaseStatus) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"INSERT INTO "release_status" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_status.id)
            .bind(release_status.name)
            .bind(release_status.parent)
            .bind(release_status.child_order)
            .bind(release_status.description)
            .bind(release_status.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_status: ReleaseStatus) -> Result<ReleaseStatus> {
        query_as::<_, ReleaseStatus>(r#"UPDATE "release_status" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_status.id)
            .bind(release_status.name)
            .bind(release_status.parent)
            .bind(release_status.child_order)
            .bind(release_status.description)
            .bind(release_status.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_status" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
