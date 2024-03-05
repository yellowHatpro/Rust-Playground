#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventSeries;

pub struct EventSeriesSet;

impl EventSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventSeries>> {
        query_as::<_, EventSeries>(r#"SELECT * FROM "musicbrainz"."event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_series: EventSeries) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"INSERT INTO "event_series" ("event", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(event_series.event)
            .bind(event_series.link)
            .bind(event_series.text_value)
            .bind(event_series.link_order)
            .bind(event_series.series)
            .bind(event_series.relationship)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_series: EventSeries) -> Result<EventSeries> {
        query_as::<_, EventSeries>(r#"UPDATE "event_series" SET "link" = $5, "event" = $1, "text_value" = $6, "series" = $2, "relationship" = $3, "link_order" = $4 WHERE  RETURNING *;"#)
            .bind(event_series.event)
            .bind(event_series.text_value)
            .bind(event_series.series)
            .bind(event_series.link)
            .bind(event_series.relationship)
            .bind(event_series.link_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
