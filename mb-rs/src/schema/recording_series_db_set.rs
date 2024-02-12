#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingSeries;

pub struct RecordingSeriesSet;

impl RecordingSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_series: RecordingSeries) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"INSERT INTO "recording_series" ("recording", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(recording_series.recording)
            .bind(recording_series.series)
            .bind(recording_series.relationship)
            .bind(recording_series.link_order)
            .bind(recording_series.link)
            .bind(recording_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_series: RecordingSeries) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"UPDATE "recording_series" SET "recording" = $1, "series" = $2, "relationship" = $3, "link_order" = $4, "link" = $5, "text_value" = $6 WHERE  RETURNING *;"#)
            .bind(recording_series.recording)
            .bind(recording_series.series)
            .bind(recording_series.relationship)
            .bind(recording_series.link_order)
            .bind(recording_series.link)
            .bind(recording_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
