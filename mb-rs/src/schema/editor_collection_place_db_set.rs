#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionPlace;

pub struct EditorCollectionPlaceSet;

impl EditorCollectionPlaceSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_place<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, place: i32) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "collection" = $1 AND "place" = $2"#)
            .bind(collection)
            .bind(place)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_place_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, place_list: Vec<i32>) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "collection" = ANY($1) AND "place" = ANY($2)"#)
            .bind(collection_list)
            .bind(place_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_place_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, place: i32) -> Result<Option<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "collection" = $1 AND "place" = $2"#)
            .bind(collection)
            .bind(place)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_id<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<EditorCollectionPlace>> {
        query_as::<_, EditorCollectionPlace>(r#"SELECT * FROM "musicbrainz"."editor_collection_place" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_place: EditorCollectionPlace) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"INSERT INTO "editor_collection_place" ("collection", "place", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_place.collection)
            .bind(editor_collection_place.place)
            .bind(editor_collection_place.added)
            .bind(editor_collection_place.position)
            .bind(editor_collection_place.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_place: EditorCollectionPlace) -> Result<EditorCollectionPlace> {
        query_as::<_, EditorCollectionPlace>(r#"UPDATE "editor_collection_place" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "place" = 2 RETURNING *;"#)
            .bind(editor_collection_place.collection)
            .bind(editor_collection_place.place)
            .bind(editor_collection_place.added)
            .bind(editor_collection_place.position)
            .bind(editor_collection_place.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_place" WHERE "collection" = 1 AND "place" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
