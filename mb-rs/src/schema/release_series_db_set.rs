#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseSeries;

pub struct ReleaseSeriesSet;

impl ReleaseSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_series: ReleaseSeries) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"INSERT INTO "release_series" ("release", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_series.release)
            .bind(release_series.relationship)
            .bind(release_series.link_order)
            .bind(release_series.series)
            .bind(release_series.link)
            .bind(release_series.text_value)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_series: ReleaseSeries) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"UPDATE "release_series" SET "series" = $2, "text_value" = $6, "release" = $1, "relationship" = $3, "link" = $5, "link_order" = $4 WHERE  RETURNING *;"#)
            .bind(release_series.series)
            .bind(release_series.link_order)
            .bind(release_series.release)
            .bind(release_series.link)
            .bind(release_series.relationship)
            .bind(release_series.text_value)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
