#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventGidRedirect;

pub struct EventGidRedirectSet;

impl EventGidRedirectSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, gid_list: Vec<uuid::Uuid>) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "gid" = ANY($1)"#)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<Option<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventGidRedirect>> {
        query_as::<_, EventGidRedirect>(r#"SELECT * FROM "musicbrainz"."event_gid_redirect" WHERE new_id = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_gid_redirect: EventGidRedirect) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"INSERT INTO "event_gid_redirect" ("gid", "new_id", "created") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(event_gid_redirect.gid)
            .bind(event_gid_redirect.new_id)
            .bind(event_gid_redirect.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_gid_redirect: EventGidRedirect) -> Result<EventGidRedirect> {
        query_as::<_, EventGidRedirect>(r#"UPDATE "event_gid_redirect" SET "new_id" = $2, "created" = $3 WHERE "gid" = 1 RETURNING *;"#)
            .bind(event_gid_redirect.gid)
            .bind(event_gid_redirect.new_id)
            .bind(event_gid_redirect.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_gid_redirect" WHERE "gid" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
