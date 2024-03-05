#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseEvent;

pub struct ReleaseEventSet;

impl ReleaseEventSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_event: ReleaseEvent) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"INSERT INTO "release_event" ("release", "date_year", "date_month", "date_day", "country") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_event.country)
            .bind(release_event.date_year)
            .bind(release_event.date_day)
            .bind(release_event.release)
            .bind(release_event.date_month)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_event: ReleaseEvent) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"UPDATE "release_event" SET "release" = $1, "date_day" = $4, "country" = $5, "date_year" = $2, "date_month" = $3 WHERE  RETURNING *;"#)
            .bind(release_event.country)
            .bind(release_event.date_month)
            .bind(release_event.release)
            .bind(release_event.date_day)
            .bind(release_event.date_year)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_event" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
