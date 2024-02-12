#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::GenreAnnotation;

pub struct GenreAnnotationSet;

impl GenreAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_genre_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, genre: i32, annotation: i32) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = $1 AND "annotation" = $2"#)
            .bind(genre)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_genre_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, genre_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(genre_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_genre_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, genre: i32, annotation: i32) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = $1 AND "annotation" = $2"#)
            .bind(genre)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_genre_id<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE genre = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, genre_annotation: GenreAnnotation) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"INSERT INTO "genre_annotation" ("genre", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(genre_annotation.genre)
            .bind(genre_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, genre_annotation: GenreAnnotation) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"UPDATE "genre_annotation" SET  WHERE "genre" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(genre_annotation.genre)
            .bind(genre_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."genre_annotation" WHERE "genre" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
