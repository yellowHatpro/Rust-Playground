#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistReleaseGroupNonva;

pub struct ArtistReleaseGroupNonvaSet;

impl ArtistReleaseGroupNonvaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistReleaseGroupNonva>> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_group_nonva""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupNonva>> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_group_nonva" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistReleaseGroupNonva>> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_group_nonva" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ArtistReleaseGroupNonva>> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"SELECT * FROM "musicbrainz"."artist_release_group_nonva" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_nonva: ArtistReleaseGroupNonva) -> Result<ArtistReleaseGroupNonva> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"INSERT INTO "artist_release_group_nonva" ("is_track_artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "unofficial", "primary_type", "secondary_types", "first_release_date", "sort_character", "release_group", "release_group", "release_group", "release_group", "release_group", "release_group", "release_group", "release_group", "release_group") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING *;"#)
            .bind(artist_release_group_nonva.unofficial)
            .bind(artist_release_group_nonva.sort_character)
            .bind(artist_release_group_nonva.first_release_date)
            .bind(artist_release_group_nonva.secondary_types)
            .bind(artist_release_group_nonva.artist)
            .bind(artist_release_group_nonva.primary_type)
            .bind(artist_release_group_nonva.release_group)
            .bind(artist_release_group_nonva.is_track_artist)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_nonva: ArtistReleaseGroupNonva) -> Result<ArtistReleaseGroupNonva> {
        query_as::<_, ArtistReleaseGroupNonva>(r#"UPDATE "artist_release_group_nonva" SET "artist" = $10, "sort_character" = $15, "release_group" = $22, "release_group" = $16, "artist" = $7, "artist" = $6, "release_group" = $19, "release_group" = $24, "artist" = $4, "artist" = $9, "artist" = $2, "artist" = $3, "artist" = $5, "artist" = $8, "release_group" = $18, "primary_type" = $12, "first_release_date" = $14, "secondary_types" = $13, "is_track_artist" = $1, "release_group" = $20, "release_group" = $17, "release_group" = $21, "release_group" = $23, "unofficial" = $11 WHERE  RETURNING *;"#)
            .bind(artist_release_group_nonva.artist)
            .bind(artist_release_group_nonva.secondary_types)
            .bind(artist_release_group_nonva.release_group)
            .bind(artist_release_group_nonva.first_release_date)
            .bind(artist_release_group_nonva.primary_type)
            .bind(artist_release_group_nonva.sort_character)
            .bind(artist_release_group_nonva.is_track_artist)
            .bind(artist_release_group_nonva.unofficial)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release_group_nonva" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
