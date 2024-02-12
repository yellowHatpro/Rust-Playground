#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionSeries;

pub struct EditorCollectionSeriesSet;

impl EditorCollectionSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_series<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, series: i32) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "collection" = $1 AND "series" = $2"#)
            .bind(collection)
            .bind(series)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_series_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, series_list: Vec<i32>) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "collection" = ANY($1) AND "series" = ANY($2)"#)
            .bind(collection_list)
            .bind(series_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_series_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, series: i32) -> Result<Option<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "collection" = $1 AND "series" = $2"#)
            .bind(collection)
            .bind(series)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_id<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<EditorCollectionSeries>> {
        query_as::<_, EditorCollectionSeries>(r#"SELECT * FROM "musicbrainz"."editor_collection_series" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_series: EditorCollectionSeries) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"INSERT INTO "editor_collection_series" ("collection", "series", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_series.collection)
            .bind(editor_collection_series.series)
            .bind(editor_collection_series.added)
            .bind(editor_collection_series.position)
            .bind(editor_collection_series.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_series: EditorCollectionSeries) -> Result<EditorCollectionSeries> {
        query_as::<_, EditorCollectionSeries>(r#"UPDATE "editor_collection_series" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "series" = 2 RETURNING *;"#)
            .bind(editor_collection_series.collection)
            .bind(editor_collection_series.series)
            .bind(editor_collection_series.added)
            .bind(editor_collection_series.position)
            .bind(editor_collection_series.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_series" WHERE "collection" = 1 AND "series" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
