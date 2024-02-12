#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistRelease;

pub struct ArtistReleaseSet;

impl ArtistReleaseSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ArtistRelease>> {
        query_as::<_, ArtistRelease>(r#"SELECT * FROM "musicbrainz"."artist_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release: ArtistRelease) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"INSERT INTO "artist_release" ("is_track_artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "artist", "first_release_date", "catalog_numbers", "country_code", "barcode", "sort_character", "release", "release", "release", "release", "release", "release", "release", "release", "release") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING *;"#)
            .bind(artist_release.is_track_artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.first_release_date)
            .bind(artist_release.catalog_numbers)
            .bind(artist_release.country_code)
            .bind(artist_release.barcode)
            .bind(artist_release.sort_character)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release: ArtistRelease) -> Result<ArtistRelease> {
        query_as::<_, ArtistRelease>(r#"UPDATE "artist_release" SET "is_track_artist" = $1, "artist" = $2, "artist" = $3, "artist" = $4, "artist" = $5, "artist" = $6, "artist" = $7, "artist" = $8, "artist" = $9, "artist" = $10, "first_release_date" = $11, "catalog_numbers" = $12, "country_code" = $13, "barcode" = $14, "sort_character" = $15, "release" = $16, "release" = $17, "release" = $18, "release" = $19, "release" = $20, "release" = $21, "release" = $22, "release" = $23, "release" = $24 WHERE  RETURNING *;"#)
            .bind(artist_release.is_track_artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.artist)
            .bind(artist_release.first_release_date)
            .bind(artist_release.catalog_numbers)
            .bind(artist_release.country_code)
            .bind(artist_release.barcode)
            .bind(artist_release.sort_character)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .bind(artist_release.release)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
