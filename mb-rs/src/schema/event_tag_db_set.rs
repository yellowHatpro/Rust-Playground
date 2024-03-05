#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventTag;

pub struct EventTagSet;

impl EventTagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_event_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, tag: i32) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = $1 AND "tag" = $2"#)
            .bind(event)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_event_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(event_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_event_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, tag: i32) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = $1 AND "tag" = $2"#)
            .bind(event)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_tag: EventTag) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"INSERT INTO "event_tag" ("event", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(event_tag.tag)
            .bind(event_tag.count)
            .bind(event_tag.event)
            .bind(event_tag.last_updated)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_tag: EventTag) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"UPDATE "event_tag" SET "count" = $3, "last_updated" = $4 WHERE "event" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(event_tag.event)
            .bind(event_tag.last_updated)
            .bind(event_tag.count)
            .bind(event_tag.tag)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_tag" WHERE "tag" = 2 AND "event" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
