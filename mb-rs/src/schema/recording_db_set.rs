#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Recording;

pub struct RecordingSet;

impl RecordingSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Recording> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Recording> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Recording> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Recording> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Recording> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<Recording>> {
        query_as::<_, Recording>(r#"SELECT * FROM "musicbrainz"."recording" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording: Recording) -> Result<Recording> {
        query_as::<_, Recording>(r#"INSERT INTO "recording" ("id", "gid", "name", "artist_credit", "length", "comment", "edits_pending", "last_updated", "video") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(recording.id)
            .bind(recording.gid)
            .bind(recording.name)
            .bind(recording.artist_credit)
            .bind(recording.length)
            .bind(recording.comment)
            .bind(recording.edits_pending)
            .bind(recording.last_updated)
            .bind(recording.video)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording: Recording) -> Result<Recording> {
        query_as::<_, Recording>(r#"UPDATE "recording" SET "gid" = $2, "name" = $3, "artist_credit" = $4, "length" = $5, "comment" = $6, "edits_pending" = $7, "last_updated" = $8, "video" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(recording.id)
            .bind(recording.gid)
            .bind(recording.name)
            .bind(recording.artist_credit)
            .bind(recording.length)
            .bind(recording.comment)
            .bind(recording.edits_pending)
            .bind(recording.last_updated)
            .bind(recording.video)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
