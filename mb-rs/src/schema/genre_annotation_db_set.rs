#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::GenreAnnotation;

pub struct GenreAnnotationSet;

impl GenreAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_genre_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, genre: i32, annotation: i32) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = $1 AND "annotation" = $2"#)
            .bind(genre)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_genre_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, genre_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(genre_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_genre_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, genre: i32, annotation: i32) -> Result<Option<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE "genre" = $1 AND "annotation" = $2"#)
            .bind(genre)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_genre_id_where_genre_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE genre = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<GenreAnnotation>> {
        query_as::<_, GenreAnnotation>(r#"SELECT * FROM "musicbrainz"."genre_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, genre_annotation: GenreAnnotation) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"INSERT INTO "genre_annotation" ("genre", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(genre_annotation.genre)
            .bind(genre_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, genre_annotation: GenreAnnotation) -> Result<GenreAnnotation> {
        query_as::<_, GenreAnnotation>(r#"UPDATE "genre_annotation" SET  WHERE "genre" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(genre_annotation.annotation)
            .bind(genre_annotation.genre)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."genre_annotation" WHERE "annotation" = 2 AND "genre" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
