#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollection;

pub struct EditorCollectionSet;

impl EditorCollectionSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_collection_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_type_id: i32) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE type = $1"#)
            .bind(editor_collection_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection: EditorCollection) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"INSERT INTO "editor_collection" ("id", "gid", "editor", "name", "public", "description", "type") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(editor_collection.public)
            .bind(editor_collection.description)
            .bind(editor_collection.Type)
            .bind(editor_collection.editor)
            .bind(editor_collection.id)
            .bind(editor_collection.gid)
            .bind(editor_collection.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection: EditorCollection) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"UPDATE "editor_collection" SET "public" = $5, "type" = $7, "name" = $4, "gid" = $2, "description" = $6, "editor" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_collection.description)
            .bind(editor_collection.editor)
            .bind(editor_collection.name)
            .bind(editor_collection.gid)
            .bind(editor_collection.public)
            .bind(editor_collection.Type)
            .bind(editor_collection.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
