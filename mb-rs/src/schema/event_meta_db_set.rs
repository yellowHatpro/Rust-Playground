#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventMeta;

pub struct EventMetaSet;

impl EventMetaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventMeta>> {
        query_as::<_, EventMeta>(r#"SELECT * FROM "musicbrainz"."event_meta" WHERE id = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_meta: EventMeta) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"INSERT INTO "event_meta" ("id", "rating", "rating_count", "event_art_presence") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(event_meta.id)
            .bind(event_meta.rating)
            .bind(event_meta.rating_count)
            .bind(event_meta.event_art_presence)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_meta: EventMeta) -> Result<EventMeta> {
        query_as::<_, EventMeta>(r#"UPDATE "event_meta" SET "rating" = $2, "rating_count" = $3, "event_art_presence" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(event_meta.id)
            .bind(event_meta.rating)
            .bind(event_meta.rating_count)
            .bind(event_meta.event_art_presence)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
