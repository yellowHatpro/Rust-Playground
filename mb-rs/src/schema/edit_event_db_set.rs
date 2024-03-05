#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditEvent;

pub struct EditEventSet;

impl EditEventSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_event<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, event: i32) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = $1 AND "event" = $2"#)
            .bind(edit)
            .bind(event)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_event_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, event_list: Vec<i32>) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = ANY($1) AND "event" = ANY($2)"#)
            .bind(edit_list)
            .bind(event_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_event_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, event: i32) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = $1 AND "event" = $2"#)
            .bind(edit)
            .bind(event)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_event: EditEvent) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"INSERT INTO "edit_event" ("edit", "event") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_event.edit)
            .bind(edit_event.event)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_event: EditEvent) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"UPDATE "edit_event" SET  WHERE "event" = 2 AND "edit" = 1 RETURNING *;"#)
            .bind(edit_event.event)
            .bind(edit_event.edit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_event" WHERE "event" = 2 AND "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
