#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Series;

pub struct SeriesSet;

impl SeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Series> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Series> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Series> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Series> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Series> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_series_type_id<'e, E: PgExecutor<'e>>(executor: E, series_type_id: i32) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE type = $1"#)
            .bind(series_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_ordering_type_id<'e, E: PgExecutor<'e>>(executor: E, series_ordering_type_id: i32) -> Result<Vec<Series>> {
        query_as::<_, Series>(r#"SELECT * FROM "musicbrainz"."series" WHERE ordering_type = $1"#)
            .bind(series_ordering_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series: Series) -> Result<Series> {
        query_as::<_, Series>(r#"INSERT INTO "series" ("id", "gid", "name", "comment", "type", "ordering_type", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(series.id)
            .bind(series.gid)
            .bind(series.name)
            .bind(series.comment)
            .bind(series.Type)
            .bind(series.ordering_type)
            .bind(series.edits_pending)
            .bind(series.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series: Series) -> Result<Series> {
        query_as::<_, Series>(r#"UPDATE "series" SET "gid" = $2, "name" = $3, "comment" = $4, "type" = $5, "ordering_type" = $6, "edits_pending" = $7, "last_updated" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(series.id)
            .bind(series.gid)
            .bind(series.name)
            .bind(series.comment)
            .bind(series.Type)
            .bind(series.ordering_type)
            .bind(series.edits_pending)
            .bind(series.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
