#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeCollection;

pub struct EditorSubscribeCollectionSet;

impl EditorSubscribeCollectionSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeCollection>> {
        query_as::<_, EditorSubscribeCollection>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_collection""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorSubscribeCollection> {
        query_as::<_, EditorSubscribeCollection>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorSubscribeCollection>> {
        query_as::<_, EditorSubscribeCollection>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_collection" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorSubscribeCollection>> {
        query_as::<_, EditorSubscribeCollection>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeCollection>> {
        query_as::<_, EditorSubscribeCollection>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_collection" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_collection: EditorSubscribeCollection) -> Result<EditorSubscribeCollection> {
        query_as::<_, EditorSubscribeCollection>(r#"INSERT INTO "editor_subscribe_collection" ("id", "editor", "collection", "last_edit_sent", "available", "last_seen_name") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(editor_subscribe_collection.editor)
            .bind(editor_subscribe_collection.id)
            .bind(editor_subscribe_collection.collection)
            .bind(editor_subscribe_collection.last_edit_sent)
            .bind(editor_subscribe_collection.available)
            .bind(editor_subscribe_collection.last_seen_name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_collection: EditorSubscribeCollection) -> Result<EditorSubscribeCollection> {
        query_as::<_, EditorSubscribeCollection>(r#"UPDATE "editor_subscribe_collection" SET "available" = $5, "editor" = $2, "collection" = $3, "last_seen_name" = $6, "last_edit_sent" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_subscribe_collection.id)
            .bind(editor_subscribe_collection.editor)
            .bind(editor_subscribe_collection.collection)
            .bind(editor_subscribe_collection.available)
            .bind(editor_subscribe_collection.last_seen_name)
            .bind(editor_subscribe_collection.last_edit_sent)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_collection" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
