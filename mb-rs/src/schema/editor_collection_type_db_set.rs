#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionType;

pub struct EditorCollectionTypeSet;

impl EditorCollectionTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionType>> {
        query_as::<_, EditorCollectionType>(r#"SELECT * FROM "musicbrainz"."editor_collection_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorCollectionType> {
        query_as::<_, EditorCollectionType>(r#"SELECT * FROM "musicbrainz"."editor_collection_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorCollectionType>> {
        query_as::<_, EditorCollectionType>(r#"SELECT * FROM "musicbrainz"."editor_collection_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorCollectionType>> {
        query_as::<_, EditorCollectionType>(r#"SELECT * FROM "musicbrainz"."editor_collection_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_type_id: i32) -> Result<Vec<EditorCollectionType>> {
        query_as::<_, EditorCollectionType>(r#"SELECT * FROM "musicbrainz"."editor_collection_type" WHERE parent = $1"#)
            .bind(editor_collection_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_type: EditorCollectionType) -> Result<EditorCollectionType> {
        query_as::<_, EditorCollectionType>(r#"INSERT INTO "editor_collection_type" ("id", "name", "entity_type", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(editor_collection_type.parent)
            .bind(editor_collection_type.name)
            .bind(editor_collection_type.id)
            .bind(editor_collection_type.child_order)
            .bind(editor_collection_type.description)
            .bind(editor_collection_type.entity_type)
            .bind(editor_collection_type.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_type: EditorCollectionType) -> Result<EditorCollectionType> {
        query_as::<_, EditorCollectionType>(r#"UPDATE "editor_collection_type" SET "entity_type" = $3, "parent" = $4, "child_order" = $5, "description" = $6, "gid" = $7, "name" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_collection_type.child_order)
            .bind(editor_collection_type.gid)
            .bind(editor_collection_type.parent)
            .bind(editor_collection_type.entity_type)
            .bind(editor_collection_type.id)
            .bind(editor_collection_type.description)
            .bind(editor_collection_type.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
