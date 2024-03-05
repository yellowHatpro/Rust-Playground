#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventAlias;

pub struct EventAliasSet;

impl EventAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventAlias>> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EventAlias> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EventAlias>> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EventAlias>> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventAlias>> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, event_alias_type_id: i32) -> Result<Vec<EventAlias>> {
        query_as::<_, EventAlias>(r#"SELECT * FROM "musicbrainz"."event_alias" WHERE type = $1"#)
            .bind(event_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_alias: EventAlias) -> Result<EventAlias> {
        query_as::<_, EventAlias>(r#"INSERT INTO "event_alias" ("id", "event", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(event_alias.edits_pending)
            .bind(event_alias.begin_date_month)
            .bind(event_alias.id)
            .bind(event_alias.locale)
            .bind(event_alias.end_date_day)
            .bind(event_alias.begin_date_day)
            .bind(event_alias.primary_for_locale)
            .bind(event_alias.sort_name)
            .bind(event_alias.Type)
            .bind(event_alias.event)
            .bind(event_alias.begin_date_year)
            .bind(event_alias.ended)
            .bind(event_alias.last_updated)
            .bind(event_alias.end_date_month)
            .bind(event_alias.name)
            .bind(event_alias.end_date_year)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_alias: EventAlias) -> Result<EventAlias> {
        query_as::<_, EventAlias>(r#"UPDATE "event_alias" SET "end_date_day" = $14, "ended" = $16, "name" = $3, "begin_date_day" = $11, "edits_pending" = $5, "begin_date_year" = $9, "sort_name" = $8, "end_date_year" = $12, "last_updated" = $6, "begin_date_month" = $10, "primary_for_locale" = $15, "event" = $2, "locale" = $4, "type" = $7, "end_date_month" = $13 WHERE "id" = 1 RETURNING *;"#)
            .bind(event_alias.locale)
            .bind(event_alias.sort_name)
            .bind(event_alias.end_date_month)
            .bind(event_alias.primary_for_locale)
            .bind(event_alias.begin_date_month)
            .bind(event_alias.last_updated)
            .bind(event_alias.ended)
            .bind(event_alias.id)
            .bind(event_alias.name)
            .bind(event_alias.end_date_day)
            .bind(event_alias.Type)
            .bind(event_alias.begin_date_year)
            .bind(event_alias.edits_pending)
            .bind(event_alias.end_date_year)
            .bind(event_alias.begin_date_day)
            .bind(event_alias.event)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
