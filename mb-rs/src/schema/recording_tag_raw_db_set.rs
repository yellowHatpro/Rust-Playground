#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingTagRaw;

pub struct RecordingTagRawSet;

impl RecordingTagRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, editor: i32, tag: i32) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "recording" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(recording)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_recording_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, recording_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "recording" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(recording_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, editor: i32, tag: i32) -> Result<Option<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "recording" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(recording)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<RecordingTagRaw>> {
        query_as::<_, RecordingTagRaw>(r#"SELECT * FROM "musicbrainz"."recording_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_tag_raw: RecordingTagRaw) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"INSERT INTO "recording_tag_raw" ("recording", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(recording_tag_raw.recording)
            .bind(recording_tag_raw.editor)
            .bind(recording_tag_raw.tag)
            .bind(recording_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_tag_raw: RecordingTagRaw) -> Result<RecordingTagRaw> {
        query_as::<_, RecordingTagRaw>(r#"UPDATE "recording_tag_raw" SET "is_upvote" = $4 WHERE "recording" = 1 AND "editor" = 2 AND "tag" = 3 RETURNING *;"#)
            .bind(recording_tag_raw.recording)
            .bind(recording_tag_raw.editor)
            .bind(recording_tag_raw.tag)
            .bind(recording_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_tag_raw" WHERE "recording" = 1 AND "editor" = 2 AND "tag" = 3"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
