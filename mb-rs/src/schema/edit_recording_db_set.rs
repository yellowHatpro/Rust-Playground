#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditRecording;

pub struct EditRecordingSet;

impl EditRecordingSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_recording<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, recording: i32) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = $1 AND "recording" = $2"#)
            .bind(edit)
            .bind(recording)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_recording_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, recording_list: Vec<i32>) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = ANY($1) AND "recording" = ANY($2)"#)
            .bind(edit_list)
            .bind(recording_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_recording_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, recording: i32) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "edit" = $1 AND "recording" = $2"#)
            .bind(edit)
            .bind(recording)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<EditRecording>> {
        query_as::<_, EditRecording>(r#"SELECT * FROM "musicbrainz"."edit_recording" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_recording: EditRecording) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"INSERT INTO "edit_recording" ("edit", "recording") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_recording.edit)
            .bind(edit_recording.recording)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_recording: EditRecording) -> Result<EditRecording> {
        query_as::<_, EditRecording>(r#"UPDATE "edit_recording" SET  WHERE "edit" = 1 AND "recording" = 2 RETURNING *;"#)
            .bind(edit_recording.edit)
            .bind(edit_recording.recording)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_recording" WHERE "edit" = 1 AND "recording" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
