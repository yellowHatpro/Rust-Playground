#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistReleaseNonva;

pub struct ArtistReleaseNonvaSet;

impl ArtistReleaseNonvaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistReleaseNonva>> {
        query_as::<_, ArtistReleaseNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_nonva""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseNonva>> {
        query_as::<_, ArtistReleaseNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_nonva" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistReleaseNonva>> {
        query_as::<_, ArtistReleaseNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_nonva" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistReleaseNonva>> {
        query_as::<_, ArtistReleaseNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_nonva" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_nonva: ArtistReleaseNonva) -> Result<ArtistReleaseNonva> {
        query_as::<_, ArtistReleaseNonva>(r#"INSERT INTO "artist_release_nonva" ("is_track_artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "first_release_date", "catalog_numbers", "country_code", "barcode", "sort_character", "release", "release", "release", "release", "release", "release", "release", "release", "release") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING *;"#)
            .bind(artist_release_nonva.first_release_date)
            .bind(artist_release_nonva.artist)
            .bind(artist_release_nonva.release)
            .bind(artist_release_nonva.country_code)
            .bind(artist_release_nonva.is_track_artist)
            .bind(artist_release_nonva.catalog_numbers)
            .bind(artist_release_nonva.barcode)
            .bind(artist_release_nonva.sort_character)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_nonva: ArtistReleaseNonva) -> Result<ArtistReleaseNonva> {
        query_as::<_, ArtistReleaseNonva>(r#"UPDATE "artist_release_nonva" SET "barcode" = $14, "artist" = $6, "release" = $21, "artist" = $9, "sort_character" = $15, "release" = $24, "release" = $23, "artist" = $7, "artist" = $5, "catalog_numbers" = $12, "artist" = $4, "first_release_date" = $11, "release" = $17, "release" = $18, "release" = $22, "release" = $19, "country_code" = $13, "is_track_artist" = $1, "artist" = $2, "artist" = $10, "artist" = $3, "release" = $16, "release" = $20, "artist" = $8 WHERE  RETURNING *;"#)
            .bind(artist_release_nonva.sort_character)
            .bind(artist_release_nonva.first_release_date)
            .bind(artist_release_nonva.artist)
            .bind(artist_release_nonva.country_code)
            .bind(artist_release_nonva.release)
            .bind(artist_release_nonva.catalog_numbers)
            .bind(artist_release_nonva.barcode)
            .bind(artist_release_nonva.is_track_artist)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release_nonva" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
