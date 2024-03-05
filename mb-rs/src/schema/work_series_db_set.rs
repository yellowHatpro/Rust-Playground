#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkSeries;

pub struct WorkSeriesSet;

impl WorkSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_series: WorkSeries) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"INSERT INTO "work_series" ("work", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(work_series.series)
            .bind(work_series.link)
            .bind(work_series.text_value)
            .bind(work_series.work)
            .bind(work_series.link_order)
            .bind(work_series.relationship)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_series: WorkSeries) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"UPDATE "work_series" SET "series" = $2, "work" = $1, "text_value" = $6, "relationship" = $3, "link_order" = $4, "link" = $5 WHERE  RETURNING *;"#)
            .bind(work_series.text_value)
            .bind(work_series.work)
            .bind(work_series.link_order)
            .bind(work_series.link)
            .bind(work_series.relationship)
            .bind(work_series.series)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
