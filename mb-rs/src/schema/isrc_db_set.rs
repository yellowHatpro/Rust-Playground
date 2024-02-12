#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Isrc;

pub struct IsrcSet;

impl IsrcSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, isrc: Isrc) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"INSERT INTO "isrc" ("id", "recording", "isrc", "source", "edits_pending", "created") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(isrc.id)
            .bind(isrc.recording)
            .bind(isrc.isrc)
            .bind(isrc.source)
            .bind(isrc.edits_pending)
            .bind(isrc.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, isrc: Isrc) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"UPDATE "isrc" SET "recording" = $2, "isrc" = $3, "source" = $4, "edits_pending" = $5, "created" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(isrc.id)
            .bind(isrc.recording)
            .bind(isrc.isrc)
            .bind(isrc.source)
            .bind(isrc.edits_pending)
            .bind(isrc.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."isrc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
