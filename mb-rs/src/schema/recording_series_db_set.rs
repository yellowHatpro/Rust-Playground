#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingSeries;

pub struct RecordingSeriesSet;

impl RecordingSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<RecordingSeries>> {
        query_as::<_, RecordingSeries>(r#"SELECT * FROM "musicbrainz"."recording_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_series: RecordingSeries) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"INSERT INTO "recording_series" ("recording", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(recording_series.relationship)
            .bind(recording_series.text_value)
            .bind(recording_series.link_order)
            .bind(recording_series.recording)
            .bind(recording_series.series)
            .bind(recording_series.link)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_series: RecordingSeries) -> Result<RecordingSeries> {
        query_as::<_, RecordingSeries>(r#"UPDATE "recording_series" SET "link_order" = $4, "text_value" = $6, "series" = $2, "link" = $5, "relationship" = $3, "recording" = $1 WHERE  RETURNING *;"#)
            .bind(recording_series.text_value)
            .bind(recording_series.relationship)
            .bind(recording_series.recording)
            .bind(recording_series.series)
            .bind(recording_series.link_order)
            .bind(recording_series.link)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
