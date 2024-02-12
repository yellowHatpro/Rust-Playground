#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkSeries;

pub struct WorkSeriesSet;

impl WorkSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkSeries>> {
        query_as::<_, WorkSeries>(r#"SELECT * FROM "musicbrainz"."work_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_series: WorkSeries) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"INSERT INTO "work_series" ("work", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(work_series.work)
            .bind(work_series.series)
            .bind(work_series.relationship)
            .bind(work_series.link_order)
            .bind(work_series.link)
            .bind(work_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_series: WorkSeries) -> Result<WorkSeries> {
        query_as::<_, WorkSeries>(r#"UPDATE "work_series" SET "work" = $1, "series" = $2, "relationship" = $3, "link_order" = $4, "link" = $5, "text_value" = $6 WHERE  RETURNING *;"#)
            .bind(work_series.work)
            .bind(work_series.series)
            .bind(work_series.relationship)
            .bind(work_series.link_order)
            .bind(work_series.link)
            .bind(work_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
