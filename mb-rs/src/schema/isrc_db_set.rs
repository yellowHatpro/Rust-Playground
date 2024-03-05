#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Isrc;

pub struct IsrcSet;

impl IsrcSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<Isrc>> {
        query_as::<_, Isrc>(r#"SELECT * FROM "musicbrainz"."isrc" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, isrc: Isrc) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"INSERT INTO "isrc" ("id", "recording", "isrc", "source", "edits_pending", "created") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(isrc.id)
            .bind(isrc.isrc)
            .bind(isrc.recording)
            .bind(isrc.source)
            .bind(isrc.edits_pending)
            .bind(isrc.created)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, isrc: Isrc) -> Result<Isrc> {
        query_as::<_, Isrc>(r#"UPDATE "isrc" SET "created" = $6, "isrc" = $3, "source" = $4, "recording" = $2, "edits_pending" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(isrc.id)
            .bind(isrc.isrc)
            .bind(isrc.created)
            .bind(isrc.edits_pending)
            .bind(isrc.source)
            .bind(isrc.recording)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."isrc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
