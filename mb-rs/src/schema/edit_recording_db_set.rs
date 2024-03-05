#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditRecording;

pub struct EditRecordingSet;

impl EditRecordingSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_recording<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, recording: i32) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = $1 AND "recording" = $2"#)
            .bind(edit)
            .bind(recording)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_recording_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, recording_list: Vec<i32>) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = ANY($1) AND "recording" = ANY($2)"#)
            .bind(edit_list)
            .bind(recording_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_recording_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, recording: i32) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = $1 AND "recording" = $2"#)
            .bind(edit)
            .bind(recording)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_recording: EditRecording) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"INSERT INTO "edit_recording" ("edit", "recording") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_recording.recording)
            .bind(edit_recording.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_recording: EditRecording) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"UPDATE "edit_recording" SET  WHERE "edit" = 1 AND "recording" = 2 RETURNING *;"#)
            .bind(edit_recording.edit)
            .bind(edit_recording.recording)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_recording" WHERE "edit" = 1 AND "recording" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
