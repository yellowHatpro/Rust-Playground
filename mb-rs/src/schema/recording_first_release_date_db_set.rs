#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingFirstReleaseDate;

pub struct RecordingFirstReleaseDateSet;

impl RecordingFirstReleaseDateSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "recording" = $1"#)
            .bind(recording)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_recording_list<'e, E: PgExecutor<'e>>(&self, executor: E, recording_list: Vec<i32>) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "recording" = ANY($1)"#)
            .bind(recording_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recording_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32) -> Result<Option<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "recording" = $1"#)
            .bind(recording)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingFirstReleaseDate>> {
        query_as::<_, RecordingFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."recording_first_release_date" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_first_release_date: RecordingFirstReleaseDate) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"INSERT INTO "recording_first_release_date" ("recording", "year", "month", "day") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(recording_first_release_date.recording)
            .bind(recording_first_release_date.year)
            .bind(recording_first_release_date.month)
            .bind(recording_first_release_date.day)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_first_release_date: RecordingFirstReleaseDate) -> Result<RecordingFirstReleaseDate> {
        query_as::<_, RecordingFirstReleaseDate>(r#"UPDATE "recording_first_release_date" SET "year" = $2, "month" = $3, "day" = $4 WHERE "recording" = 1 RETURNING *;"#)
            .bind(recording_first_release_date.recording)
            .bind(recording_first_release_date.year)
            .bind(recording_first_release_date.month)
            .bind(recording_first_release_date.day)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_first_release_date" WHERE "recording" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
