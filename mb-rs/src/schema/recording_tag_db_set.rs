#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingTag;

pub struct RecordingTagSet;

impl RecordingTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, tag: i32) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "recording" = $1 AND "tag" = $2"#)
            .bind(recording)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_recording_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, recording_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "recording" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(recording_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, tag: i32) -> Result<Option<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "recording" = $1 AND "tag" = $2"#)
            .bind(recording)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<RecordingTag>> {
        query_as::<_, RecordingTag>(r#"SELECT * FROM "musicbrainz"."recording_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_tag: RecordingTag) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"INSERT INTO "recording_tag" ("recording", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(recording_tag.recording)
            .bind(recording_tag.tag)
            .bind(recording_tag.count)
            .bind(recording_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_tag: RecordingTag) -> Result<RecordingTag> {
        query_as::<_, RecordingTag>(r#"UPDATE "recording_tag" SET "count" = $3, "last_updated" = $4 WHERE "recording" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(recording_tag.recording)
            .bind(recording_tag.tag)
            .bind(recording_tag.count)
            .bind(recording_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_tag" WHERE "recording" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
