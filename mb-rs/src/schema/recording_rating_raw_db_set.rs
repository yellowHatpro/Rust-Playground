#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingRatingRaw;

pub struct RecordingRatingRawSet;

impl RecordingRatingRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingRatingRaw>> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_recording_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, editor: i32) -> Result<RecordingRatingRaw> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw" WHERE "recording" = $1 AND "editor" = $2"#)
            .bind(recording)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_recording_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, recording_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<RecordingRatingRaw>> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw" WHERE "recording" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(recording_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_recording_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, editor: i32) -> Result<Option<RecordingRatingRaw>> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw" WHERE "recording" = $1 AND "editor" = $2"#)
            .bind(recording)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingRatingRaw>> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<RecordingRatingRaw>> {
        query_as::<_, RecordingRatingRaw>(r#"SELECT * FROM "musicbrainz"."recording_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_rating_raw: RecordingRatingRaw) -> Result<RecordingRatingRaw> {
        query_as::<_, RecordingRatingRaw>(r#"INSERT INTO "recording_rating_raw" ("recording", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(recording_rating_raw.recording)
            .bind(recording_rating_raw.editor)
            .bind(recording_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_rating_raw: RecordingRatingRaw) -> Result<RecordingRatingRaw> {
        query_as::<_, RecordingRatingRaw>(r#"UPDATE "recording_rating_raw" SET "rating" = $3 WHERE "recording" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(recording_rating_raw.recording)
            .bind(recording_rating_raw.editor)
            .bind(recording_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_rating_raw" WHERE "editor" = 2 AND "recording" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
