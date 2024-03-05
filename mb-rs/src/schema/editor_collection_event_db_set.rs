#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionEvent;

pub struct EditorCollectionEventSet;

impl EditorCollectionEventSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_event<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, event: i32) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = $1 AND "event" = $2"#)
            .bind(collection)
            .bind(event)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_event_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, event_list: Vec<i32>) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = ANY($1) AND "event" = ANY($2)"#)
            .bind(collection_list)
            .bind(event_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_event_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, event: i32) -> Result<Option<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE "collection" = $1 AND "event" = $2"#)
            .bind(collection)
            .bind(event)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EditorCollectionEvent>> {
        query_as::<_, EditorCollectionEvent>(r#"SELECT * FROM "musicbrainz"."editor_collection_event" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_event: EditorCollectionEvent) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"INSERT INTO "editor_collection_event" ("collection", "event", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_event.comment)
            .bind(editor_collection_event.position)
            .bind(editor_collection_event.event)
            .bind(editor_collection_event.added)
            .bind(editor_collection_event.collection)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_event: EditorCollectionEvent) -> Result<EditorCollectionEvent> {
        query_as::<_, EditorCollectionEvent>(r#"UPDATE "editor_collection_event" SET "position" = $4, "added" = $3, "comment" = $5 WHERE "collection" = 1 AND "event" = 2 RETURNING *;"#)
            .bind(editor_collection_event.collection)
            .bind(editor_collection_event.position)
            .bind(editor_collection_event.event)
            .bind(editor_collection_event.comment)
            .bind(editor_collection_event.added)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_event" WHERE "collection" = 1 AND "event" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
