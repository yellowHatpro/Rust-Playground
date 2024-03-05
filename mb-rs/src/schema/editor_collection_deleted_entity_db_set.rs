#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionDeletedEntity;

pub struct EditorCollectionDeletedEntitySet;

impl EditorCollectionDeletedEntitySet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_gid<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, gid: uuid::Uuid) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = $1 AND "gid" = $2"#)
            .bind(collection)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, gid_list: Vec<uuid::Uuid>) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = ANY($1) AND "gid" = ANY($2)"#)
            .bind(collection_list)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, gid: uuid::Uuid) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = $1 AND "gid" = $2"#)
            .bind(collection)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_deleted_entity_gid_where_gid_is<'e, E: PgExecutor<'e>>(executor: E, deleted_entity_gid: uuid::Uuid) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE gid = $1"#)
            .bind(deleted_entity_gid)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_deleted_entity: EditorCollectionDeletedEntity) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"INSERT INTO "editor_collection_deleted_entity" ("collection", "gid", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_deleted_entity.added)
            .bind(editor_collection_deleted_entity.collection)
            .bind(editor_collection_deleted_entity.comment)
            .bind(editor_collection_deleted_entity.gid)
            .bind(editor_collection_deleted_entity.position)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_deleted_entity: EditorCollectionDeletedEntity) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"UPDATE "editor_collection_deleted_entity" SET "comment" = $5, "position" = $4, "added" = $3 WHERE "collection" = 1 AND "gid" = 2 RETURNING *;"#)
            .bind(editor_collection_deleted_entity.comment)
            .bind(editor_collection_deleted_entity.added)
            .bind(editor_collection_deleted_entity.collection)
            .bind(editor_collection_deleted_entity.position)
            .bind(editor_collection_deleted_entity.gid)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "gid" = 2 AND "collection" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
