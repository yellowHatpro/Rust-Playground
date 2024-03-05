#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Event;

pub struct EventSet;

impl EventSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Event> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, event_type_id: i32) -> Result<Vec<Event>> {
        query_as::<_, Event>(r#"SELECT * FROM "musicbrainz"."event" WHERE type = $1"#)
            .bind(event_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event: Event) -> Result<Event> {
        query_as::<_, Event>(r#"INSERT INTO "event" ("id", "gid", "name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "time", "type", "cancelled", "setlist", "comment", "edits_pending", "last_updated", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(event.gid)
            .bind(event.end_date_month)
            .bind(event.end_date_day)
            .bind(event.edits_pending)
            .bind(event.begin_date_month)
            .bind(event.Type)
            .bind(event.setlist)
            .bind(event.cancelled)
            .bind(event.ended)
            .bind(event.begin_date_year)
            .bind(event.id)
            .bind(event.comment)
            .bind(event.end_date_year)
            .bind(event.name)
            .bind(event.begin_date_day)
            .bind(event.time)
            .bind(event.last_updated)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event: Event) -> Result<Event> {
        query_as::<_, Event>(r#"UPDATE "event" SET "name" = $3, "gid" = $2, "end_date_month" = $8, "type" = $11, "time" = $10, "end_date_year" = $7, "end_date_day" = $9, "last_updated" = $16, "edits_pending" = $15, "comment" = $14, "cancelled" = $12, "begin_date_month" = $5, "begin_date_day" = $6, "setlist" = $13, "begin_date_year" = $4, "ended" = $17 WHERE "id" = 1 RETURNING *;"#)
            .bind(event.begin_date_year)
            .bind(event.begin_date_day)
            .bind(event.last_updated)
            .bind(event.end_date_day)
            .bind(event.time)
            .bind(event.edits_pending)
            .bind(event.setlist)
            .bind(event.end_date_month)
            .bind(event.comment)
            .bind(event.ended)
            .bind(event.Type)
            .bind(event.id)
            .bind(event.end_date_year)
            .bind(event.gid)
            .bind(event.begin_date_month)
            .bind(event.name)
            .bind(event.cancelled)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
