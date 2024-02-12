#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistReleaseGroupPendingUpdate;

pub struct ArtistReleaseGroupPendingUpdateSet;

impl ArtistReleaseGroupPendingUpdateSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_pending_update: ArtistReleaseGroupPendingUpdate) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"INSERT INTO "artist_release_group_pending_update" ("release_group") VALUES ($1) RETURNING *;"#)
            .bind(artist_release_group_pending_update.release_group)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_pending_update: ArtistReleaseGroupPendingUpdate) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"UPDATE "artist_release_group_pending_update" SET "release_group" = $1 WHERE  RETURNING *;"#)
            .bind(artist_release_group_pending_update.release_group)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
