#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistReleaseGroupPendingUpdate;

pub struct ArtistReleaseGroupPendingUpdateSet;

impl ArtistReleaseGroupPendingUpdateSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistReleaseGroupPendingUpdate>> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"SELECT * FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_pending_update: ArtistReleaseGroupPendingUpdate) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"INSERT INTO "artist_release_group_pending_update" ("release_group") VALUES ($1) RETURNING *;"#)
            .bind(artist_release_group_pending_update.release_group)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_release_group_pending_update: ArtistReleaseGroupPendingUpdate) -> Result<ArtistReleaseGroupPendingUpdate> {
        query_as::<_, ArtistReleaseGroupPendingUpdate>(r#"UPDATE "artist_release_group_pending_update" SET "release_group" = $1 WHERE  RETURNING *;"#)
            .bind(artist_release_group_pending_update.release_group)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_release_group_pending_update" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
