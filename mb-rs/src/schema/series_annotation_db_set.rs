#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesAnnotation;

pub struct SeriesAnnotationSet;

impl SeriesAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_series_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, series: i32, annotation: i32) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "series" = $1 AND "annotation" = $2"#)
            .bind(series)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_series_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, series_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "series" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(series_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_series_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, series: i32, annotation: i32) -> Result<Option<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "series" = $1 AND "annotation" = $2"#)
            .bind(series)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_series_id<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<SeriesAnnotation>> {
        query_as::<_, SeriesAnnotation>(r#"SELECT * FROM "musicbrainz"."series_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_annotation: SeriesAnnotation) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"INSERT INTO "series_annotation" ("series", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(series_annotation.series)
            .bind(series_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_annotation: SeriesAnnotation) -> Result<SeriesAnnotation> {
        query_as::<_, SeriesAnnotation>(r#"UPDATE "series_annotation" SET  WHERE "series" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(series_annotation.series)
            .bind(series_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_annotation" WHERE "series" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
