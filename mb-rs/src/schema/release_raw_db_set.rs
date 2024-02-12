#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseRaw;

pub struct ReleaseRawSet;

impl ReleaseRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_raw: ReleaseRaw) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"INSERT INTO "release_raw" ("id", "title", "artist", "added", "last_modified", "lookup_count", "modify_count", "source", "barcode", "comment") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;"#)
            .bind(release_raw.id)
            .bind(release_raw.title)
            .bind(release_raw.artist)
            .bind(release_raw.added)
            .bind(release_raw.last_modified)
            .bind(release_raw.lookup_count)
            .bind(release_raw.modify_count)
            .bind(release_raw.source)
            .bind(release_raw.barcode)
            .bind(release_raw.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_raw: ReleaseRaw) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"UPDATE "release_raw" SET "title" = $2, "artist" = $3, "added" = $4, "last_modified" = $5, "lookup_count" = $6, "modify_count" = $7, "source" = $8, "barcode" = $9, "comment" = $10 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_raw.id)
            .bind(release_raw.title)
            .bind(release_raw.artist)
            .bind(release_raw.added)
            .bind(release_raw.last_modified)
            .bind(release_raw.lookup_count)
            .bind(release_raw.modify_count)
            .bind(release_raw.source)
            .bind(release_raw.barcode)
            .bind(release_raw.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_raw" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
