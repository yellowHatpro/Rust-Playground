#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseEvent;

pub struct ReleaseEventSet;

impl ReleaseEventSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseEvent>> {
        query_as::<_, ReleaseEvent>(r#"SELECT * FROM "musicbrainz"."release_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_event: ReleaseEvent) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"INSERT INTO "release_event" ("release", "date_year", "date_month", "date_day", "country") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_event.release)
            .bind(release_event.date_year)
            .bind(release_event.date_month)
            .bind(release_event.date_day)
            .bind(release_event.country)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_event: ReleaseEvent) -> Result<ReleaseEvent> {
        query_as::<_, ReleaseEvent>(r#"UPDATE "release_event" SET "release" = $1, "date_year" = $2, "date_month" = $3, "date_day" = $4, "country" = $5 WHERE  RETURNING *;"#)
            .bind(release_event.release)
            .bind(release_event.date_year)
            .bind(release_event.date_month)
            .bind(release_event.date_day)
            .bind(release_event.country)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_event" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
