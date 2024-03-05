#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventRatingRaw;

pub struct EventRatingRawSet;

impl EventRatingRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventRatingRaw>> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_event_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, editor: i32) -> Result<EventRatingRaw> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw" WHERE "event" = $1 AND "editor" = $2"#)
            .bind(event)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_event_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<EventRatingRaw>> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw" WHERE "event" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(event_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_event_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, editor: i32) -> Result<Option<EventRatingRaw>> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw" WHERE "event" = $1 AND "editor" = $2"#)
            .bind(event)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventRatingRaw>> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EventRatingRaw>> {
        query_as::<_, EventRatingRaw>(r#"SELECT * FROM "musicbrainz"."event_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_rating_raw: EventRatingRaw) -> Result<EventRatingRaw> {
        query_as::<_, EventRatingRaw>(r#"INSERT INTO "event_rating_raw" ("event", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(event_rating_raw.event)
            .bind(event_rating_raw.rating)
            .bind(event_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_rating_raw: EventRatingRaw) -> Result<EventRatingRaw> {
        query_as::<_, EventRatingRaw>(r#"UPDATE "event_rating_raw" SET "rating" = $3 WHERE "editor" = 2 AND "event" = 1 RETURNING *;"#)
            .bind(event_rating_raw.event)
            .bind(event_rating_raw.editor)
            .bind(event_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_rating_raw" WHERE "editor" = 2 AND "event" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
