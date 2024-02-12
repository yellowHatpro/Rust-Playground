#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollection;

pub struct EditorCollectionSet;

impl EditorCollectionSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_collection_type_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_type_id: i32) -> Result<Vec<EditorCollection>> {
        query_as::<_, EditorCollection>(r#"SELECT * FROM "musicbrainz"."editor_collection" WHERE type = $1"#)
            .bind(editor_collection_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection: EditorCollection) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"INSERT INTO "editor_collection" ("id", "gid", "editor", "name", "public", "description", "type") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(editor_collection.id)
            .bind(editor_collection.gid)
            .bind(editor_collection.editor)
            .bind(editor_collection.name)
            .bind(editor_collection.public)
            .bind(editor_collection.description)
            .bind(editor_collection.Type)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection: EditorCollection) -> Result<EditorCollection> {
        query_as::<_, EditorCollection>(r#"UPDATE "editor_collection" SET "gid" = $2, "editor" = $3, "name" = $4, "public" = $5, "description" = $6, "type" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_collection.id)
            .bind(editor_collection.gid)
            .bind(editor_collection.editor)
            .bind(editor_collection.name)
            .bind(editor_collection.public)
            .bind(editor_collection.description)
            .bind(editor_collection.Type)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
