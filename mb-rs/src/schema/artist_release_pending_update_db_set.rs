#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistReleasePendingUpdate;

pub struct ArtistReleasePendingUpdateSet;

impl ArtistReleasePendingUpdateSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleasePendingUpdate>> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_pending_update: ArtistReleasePendingUpdate) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"INSERT INTO "artist_release_pending_update" ("release") VALUES ($1) RETURNING *;"#)
            .bind(artist_release_pending_update.release)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_pending_update: ArtistReleasePendingUpdate) -> Result<ArtistReleasePendingUpdate> {
        query_as::<_, ArtistReleasePendingUpdate>(r#"UPDATE "artist_release_pending_update" SET "release" = $1 WHERE  RETURNING *;"#)
            .bind(artist_release_pending_update.release)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release_pending_update" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
