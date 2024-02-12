#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionRecording;

pub struct EditorCollectionRecordingSet;

impl EditorCollectionRecordingSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_recording<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, recording: i32) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "collection" = $1 AND "recording" = $2"#)
            .bind(collection)
            .bind(recording)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_recording_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, recording_list: Vec<i32>) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "collection" = ANY($1) AND "recording" = ANY($2)"#)
            .bind(collection_list)
            .bind(recording_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_recording_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, recording: i32) -> Result<Option<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "collection" = $1 AND "recording" = $2"#)
            .bind(collection)
            .bind(recording)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<EditorCollectionRecording>> {
        query_as::<_, EditorCollectionRecording>(r#"SELECT * FROM "musicbrainz"."editor_collection_recording" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_recording: EditorCollectionRecording) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"INSERT INTO "editor_collection_recording" ("collection", "recording", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_recording.collection)
            .bind(editor_collection_recording.recording)
            .bind(editor_collection_recording.added)
            .bind(editor_collection_recording.position)
            .bind(editor_collection_recording.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_recording: EditorCollectionRecording) -> Result<EditorCollectionRecording> {
        query_as::<_, EditorCollectionRecording>(r#"UPDATE "editor_collection_recording" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "recording" = 2 RETURNING *;"#)
            .bind(editor_collection_recording.collection)
            .bind(editor_collection_recording.recording)
            .bind(editor_collection_recording.added)
            .bind(editor_collection_recording.position)
            .bind(editor_collection_recording.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_recording" WHERE "collection" = 1 AND "recording" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
