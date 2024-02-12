#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventSeries;

pub struct EventSeriesSet;

impl EventSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_series: EventSeries) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"INSERT INTO "event_series" ("event", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(event_series.event)
            .bind(event_series.series)
            .bind(event_series.relationship)
            .bind(event_series.link_order)
            .bind(event_series.link)
            .bind(event_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_series: EventSeries) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"UPDATE "event_series" SET "event" = $1, "series" = $2, "relationship" = $3, "link_order" = $4, "link" = $5, "text_value" = $6 WHERE  RETURNING *;"#)
            .bind(event_series.event)
            .bind(event_series.series)
            .bind(event_series.relationship)
            .bind(event_series.link_order)
            .bind(event_series.link)
            .bind(event_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
