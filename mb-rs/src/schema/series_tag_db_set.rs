#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesTag;

pub struct SeriesTagSet;

impl SeriesTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_series_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, series: i32, tag: i32) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "series" = $1 AND "tag" = $2"#)
            .bind(series)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_series_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, series_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "series" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(series_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_series_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, series: i32, tag: i32) -> Result<Option<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "series" = $1 AND "tag" = $2"#)
            .bind(series)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_series_id<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<SeriesTag>> {
        query_as::<_, SeriesTag>(r#"SELECT * FROM "musicbrainz"."series_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_tag: SeriesTag) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"INSERT INTO "series_tag" ("series", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(series_tag.series)
            .bind(series_tag.tag)
            .bind(series_tag.count)
            .bind(series_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_tag: SeriesTag) -> Result<SeriesTag> {
        query_as::<_, SeriesTag>(r#"UPDATE "series_tag" SET "count" = $3, "last_updated" = $4 WHERE "series" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(series_tag.series)
            .bind(series_tag.tag)
            .bind(series_tag.count)
            .bind(series_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_tag" WHERE "series" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
