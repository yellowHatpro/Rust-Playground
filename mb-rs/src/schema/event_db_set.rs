#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Event;

pub struct EventSet;

impl EventSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_event_type_id<'e, E: PgExecutor<'e>>(executor: E, event_type_id: i32) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE type = $1"#)
            .bind(event_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event: Event) -> Result<Event> {
        query_as::<_, Event>(r#"INSERT INTO "event" ("id", "gid", "name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "time", "type", "cancelled", "setlist", "comment", "edits_pending", "last_updated", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(event.id)
            .bind(event.gid)
            .bind(event.name)
            .bind(event.begin_date_year)
            .bind(event.begin_date_month)
            .bind(event.begin_date_day)
            .bind(event.end_date_year)
            .bind(event.end_date_month)
            .bind(event.end_date_day)
            .bind(event.time)
            .bind(event.Type)
            .bind(event.cancelled)
            .bind(event.setlist)
            .bind(event.comment)
            .bind(event.edits_pending)
            .bind(event.last_updated)
            .bind(event.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event: Event) -> Result<Event> {
        query_as::<_, Event>(r#"UPDATE "event" SET "gid" = $2, "name" = $3, "begin_date_year" = $4, "begin_date_month" = $5, "begin_date_day" = $6, "end_date_year" = $7, "end_date_month" = $8, "end_date_day" = $9, "time" = $10, "type" = $11, "cancelled" = $12, "setlist" = $13, "comment" = $14, "edits_pending" = $15, "last_updated" = $16, "ended" = $17 WHERE "id" = 1 RETURNING *;"#)
            .bind(event.id)
            .bind(event.gid)
            .bind(event.name)
            .bind(event.begin_date_year)
            .bind(event.begin_date_month)
            .bind(event.begin_date_day)
            .bind(event.end_date_year)
            .bind(event.end_date_month)
            .bind(event.end_date_day)
            .bind(event.time)
            .bind(event.Type)
            .bind(event.cancelled)
            .bind(event.setlist)
            .bind(event.comment)
            .bind(event.edits_pending)
            .bind(event.last_updated)
            .bind(event.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
