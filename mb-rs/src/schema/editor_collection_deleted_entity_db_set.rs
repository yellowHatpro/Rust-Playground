#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionDeletedEntity;

pub struct EditorCollectionDeletedEntitySet;

impl EditorCollectionDeletedEntitySet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_gid<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, gid: uuid::Uuid) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = $1 AND "gid" = $2"#)
            .bind(collection)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, gid_list: Vec<uuid::Uuid>) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = ANY($1) AND "gid" = ANY($2)"#)
            .bind(collection_list)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, gid: uuid::Uuid) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = $1 AND "gid" = $2"#)
            .bind(collection)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_deleted_entity_gid<'e, E: PgExecutor<'e>>(executor: E, deleted_entity_gid: uuid::Uuid) -> Result<Vec<EditorCollectionDeletedEntity>> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"SELECT * FROM "musicbrainz"."editor_collection_deleted_entity" WHERE gid = $1"#)
            .bind(deleted_entity_gid)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_deleted_entity: EditorCollectionDeletedEntity) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"INSERT INTO "editor_collection_deleted_entity" ("collection", "gid", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_deleted_entity.collection)
            .bind(editor_collection_deleted_entity.gid)
            .bind(editor_collection_deleted_entity.added)
            .bind(editor_collection_deleted_entity.position)
            .bind(editor_collection_deleted_entity.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_deleted_entity: EditorCollectionDeletedEntity) -> Result<EditorCollectionDeletedEntity> {
        query_as::<_, EditorCollectionDeletedEntity>(r#"UPDATE "editor_collection_deleted_entity" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "gid" = 2 RETURNING *;"#)
            .bind(editor_collection_deleted_entity.collection)
            .bind(editor_collection_deleted_entity.gid)
            .bind(editor_collection_deleted_entity.added)
            .bind(editor_collection_deleted_entity.position)
            .bind(editor_collection_deleted_entity.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_deleted_entity" WHERE "collection" = 1 AND "gid" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
