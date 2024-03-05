#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistAnnotation;

pub struct ArtistAnnotationSet;

impl ArtistAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_artist_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, annotation: i32) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = $1 AND "annotation" = $2"#)
            .bind(artist)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_artist_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(artist_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_artist_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, annotation: i32) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = $1 AND "annotation" = $2"#)
            .bind(artist)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_annotation: ArtistAnnotation) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"INSERT INTO "artist_annotation" ("artist", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(artist_annotation.artist)
            .bind(artist_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_annotation: ArtistAnnotation) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"UPDATE "artist_annotation" SET  WHERE "annotation" = 2 AND "artist" = 1 RETURNING *;"#)
            .bind(artist_annotation.annotation)
            .bind(artist_annotation.artist)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_annotation" WHERE "artist" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
