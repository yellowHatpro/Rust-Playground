#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroup;

pub struct ReleaseGroupSet;

impl ReleaseGroupSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_primary_type_id<'e, E: PgExecutor<'e>>(executor: E, release_group_primary_type_id: i32) -> Result<Vec<ReleaseGroup>> {
        query_as::<_, ReleaseGroup>(r#"SELECT * FROM "musicbrainz"."release_group" WHERE type = $1"#)
            .bind(release_group_primary_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: ReleaseGroup) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"INSERT INTO "release_group" ("id", "gid", "name", "artist_credit", "type", "comment", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(release_group.id)
            .bind(release_group.gid)
            .bind(release_group.name)
            .bind(release_group.artist_credit)
            .bind(release_group.Type)
            .bind(release_group.comment)
            .bind(release_group.edits_pending)
            .bind(release_group.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: ReleaseGroup) -> Result<ReleaseGroup> {
        query_as::<_, ReleaseGroup>(r#"UPDATE "release_group" SET "gid" = $2, "name" = $3, "artist_credit" = $4, "type" = $5, "comment" = $6, "edits_pending" = $7, "last_updated" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group.id)
            .bind(release_group.gid)
            .bind(release_group.name)
            .bind(release_group.artist_credit)
            .bind(release_group.Type)
            .bind(release_group.comment)
            .bind(release_group.edits_pending)
            .bind(release_group.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
