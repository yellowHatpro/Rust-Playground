#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistAnnotation;

pub struct ArtistAnnotationSet;

impl ArtistAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, annotation: i32) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = $1 AND "annotation" = $2"#)
            .bind(artist)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_artist_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(artist_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, annotation: i32) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "artist" = $1 AND "annotation" = $2"#)
            .bind(artist)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<ArtistAnnotation>> {
        query_as::<_, ArtistAnnotation>(r#"SELECT * FROM "musicbrainz"."artist_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_annotation: ArtistAnnotation) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"INSERT INTO "artist_annotation" ("artist", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(artist_annotation.artist)
            .bind(artist_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_annotation: ArtistAnnotation) -> Result<ArtistAnnotation> {
        query_as::<_, ArtistAnnotation>(r#"UPDATE "artist_annotation" SET  WHERE "artist" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(artist_annotation.artist)
            .bind(artist_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_annotation" WHERE "artist" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
