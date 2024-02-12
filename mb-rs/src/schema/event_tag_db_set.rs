#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventTag;

pub struct EventTagSet;

impl EventTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, tag: i32) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = $1 AND "tag" = $2"#)
            .bind(event)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(event_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, tag: i32) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "event" = $1 AND "tag" = $2"#)
            .bind(event)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<EventTag>> {
        query_as::<_, EventTag>(r#"SELECT * FROM "musicbrainz"."event_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_tag: EventTag) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"INSERT INTO "event_tag" ("event", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(event_tag.event)
            .bind(event_tag.tag)
            .bind(event_tag.count)
            .bind(event_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_tag: EventTag) -> Result<EventTag> {
        query_as::<_, EventTag>(r#"UPDATE "event_tag" SET "count" = $3, "last_updated" = $4 WHERE "event" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(event_tag.event)
            .bind(event_tag.tag)
            .bind(event_tag.count)
            .bind(event_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_tag" WHERE "event" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
