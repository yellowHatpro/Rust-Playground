#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionArea;

pub struct EditorCollectionAreaSet;

impl EditorCollectionAreaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_area<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, area: i32) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = $1 AND "area" = $2"#)
            .bind(collection)
            .bind(area)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_area_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, area_list: Vec<i32>) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = ANY($1) AND "area" = ANY($2)"#)
            .bind(collection_list)
            .bind(area_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_area_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, area: i32) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "collection" = $1 AND "area" = $2"#)
            .bind(collection)
            .bind(area)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<EditorCollectionArea>> {
        query_as::<_, EditorCollectionArea>(r#"SELECT * FROM "musicbrainz"."editor_collection_area" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_area: EditorCollectionArea) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"INSERT INTO "editor_collection_area" ("collection", "area", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_area.collection)
            .bind(editor_collection_area.area)
            .bind(editor_collection_area.added)
            .bind(editor_collection_area.position)
            .bind(editor_collection_area.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_area: EditorCollectionArea) -> Result<EditorCollectionArea> {
        query_as::<_, EditorCollectionArea>(r#"UPDATE "editor_collection_area" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "area" = 2 RETURNING *;"#)
            .bind(editor_collection_area.collection)
            .bind(editor_collection_area.area)
            .bind(editor_collection_area.added)
            .bind(editor_collection_area.position)
            .bind(editor_collection_area.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_area" WHERE "collection" = 1 AND "area" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
