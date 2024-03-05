#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupSeries;

pub struct ReleaseGroupSeriesSet;

impl ReleaseGroupSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupSeries>> {
        query_as::<_, ReleaseGroupSeries>(r#"SELECT * FROM "musicbrainz"."release_group_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupSeries>> {
        query_as::<_, ReleaseGroupSeries>(r#"SELECT * FROM "musicbrainz"."release_group_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_series: ReleaseGroupSeries) -> Result<ReleaseGroupSeries> {
        query_as::<_, ReleaseGroupSeries>(r#"INSERT INTO "release_group_series" ("release_group", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_group_series.link_order)
            .bind(release_group_series.relationship)
            .bind(release_group_series.release_group)
            .bind(release_group_series.link)
            .bind(release_group_series.series)
            .bind(release_group_series.text_value)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_series: ReleaseGroupSeries) -> Result<ReleaseGroupSeries> {
        query_as::<_, ReleaseGroupSeries>(r#"UPDATE "release_group_series" SET "series" = $2, "relationship" = $3, "release_group" = $1, "link" = $5, "text_value" = $6, "link_order" = $4 WHERE  RETURNING *;"#)
            .bind(release_group_series.text_value)
            .bind(release_group_series.link_order)
            .bind(release_group_series.link)
            .bind(release_group_series.release_group)
            .bind(release_group_series.series)
            .bind(release_group_series.relationship)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
