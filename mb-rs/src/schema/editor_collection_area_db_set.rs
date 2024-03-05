#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionArea;

pub struct EditorCollectionAreaSet;

impl EditorCollectionAreaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_area<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, area: i32) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = $1 AND "area" = $2"#)
            .bind(collection)
            .bind(area)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_area_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, area_list: Vec<i32>) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = ANY($1) AND "area" = ANY($2)"#)
            .bind(collection_list)
            .bind(area_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_area_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, area: i32) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = $1 AND "area" = $2"#)
            .bind(collection)
            .bind(area)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_area: EditorCollectionArea) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"INSERT INTO "editor_collection_area" ("collection", "area", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_area.position)
            .bind(editor_collection_area.area)
            .bind(editor_collection_area.comment)
            .bind(editor_collection_area.added)
            .bind(editor_collection_area.collection)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_area: EditorCollectionArea) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"UPDATE "editor_collection_area" SET "comment" = $5, "added" = $3, "position" = $4 WHERE "collection" = 1 AND "area" = 2 RETURNING *;"#)
            .bind(editor_collection_area.added)
            .bind(editor_collection_area.comment)
            .bind(editor_collection_area.position)
            .bind(editor_collection_area.area)
            .bind(editor_collection_area.collection)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_area" WHERE "area" = 2 AND "collection" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
