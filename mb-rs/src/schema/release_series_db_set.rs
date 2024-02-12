#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseSeries;

pub struct ReleaseSeriesSet;

impl ReleaseSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseSeries>> {
        query_as::<_, ReleaseSeries>(r#"SELECT * FROM "musicbrainz"."release_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_series: ReleaseSeries) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"INSERT INTO "release_series" ("release", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_series.release)
            .bind(release_series.series)
            .bind(release_series.relationship)
            .bind(release_series.link_order)
            .bind(release_series.link)
            .bind(release_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_series: ReleaseSeries) -> Result<ReleaseSeries> {
        query_as::<_, ReleaseSeries>(r#"UPDATE "release_series" SET "release" = $1, "series" = $2, "relationship" = $3, "link_order" = $4, "link" = $5, "text_value" = $6 WHERE  RETURNING *;"#)
            .bind(release_series.release)
            .bind(release_series.series)
            .bind(release_series.relationship)
            .bind(release_series.link_order)
            .bind(release_series.link)
            .bind(release_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
