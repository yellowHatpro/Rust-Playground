#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionEvent;

pub struct EditorCollectionEventSet;

impl EditorCollectionEventSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_event<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, event: i32) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = $1 AND "event" = $2"#)
            .bind(collection)
            .bind(event)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_event_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, event_list: Vec<i32>) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = ANY($1) AND "event" = ANY($2)"#)
            .bind(collection_list)
            .bind(event_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_event_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, event: i32) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = $1 AND "event" = $2"#)
            .bind(collection)
            .bind(event)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_event: EditorCollectionEvent) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"INSERT INTO "editor_collection_event" ("collection", "event", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_event.collection)
            .bind(editor_collection_event.event)
            .bind(editor_collection_event.added)
            .bind(editor_collection_event.position)
            .bind(editor_collection_event.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_event: EditorCollectionEvent) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"UPDATE "editor_collection_event" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "event" = 2 RETURNING *;"#)
            .bind(editor_collection_event.collection)
            .bind(editor_collection_event.event)
            .bind(editor_collection_event.added)
            .bind(editor_collection_event.position)
            .bind(editor_collection_event.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_event" WHERE "collection" = 1 AND "event" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
