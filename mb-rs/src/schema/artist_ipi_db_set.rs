#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistIpi;

pub struct ArtistIpiSet;

impl ArtistIpiSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_ipi<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, ipi: char) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = $1 AND "ipi" = $2"#)
            .bind(artist)
            .bind(ipi)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_artist_and_ipi_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, ipi_list: Vec<char>) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = ANY($1) AND "ipi" = ANY($2)"#)
            .bind(artist_list)
            .bind(ipi_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_ipi_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, ipi: char) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = $1 AND "ipi" = $2"#)
            .bind(artist)
            .bind(ipi)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_ipi: ArtistIpi) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"INSERT INTO "artist_ipi" ("artist", "ipi", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(artist_ipi.artist)
            .bind(artist_ipi.ipi)
            .bind(artist_ipi.edits_pending)
            .bind(artist_ipi.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_ipi: ArtistIpi) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"UPDATE "artist_ipi" SET "edits_pending" = $3, "created" = $4 WHERE "artist" = 1 AND "ipi" = 2 RETURNING *;"#)
            .bind(artist_ipi.artist)
            .bind(artist_ipi.ipi)
            .bind(artist_ipi.edits_pending)
            .bind(artist_ipi.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_ipi" WHERE "artist" = 1 AND "ipi" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
