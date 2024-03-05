#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistIpi;

pub struct ArtistIpiSet;

impl ArtistIpiSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_artist_and_ipi<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, ipi: String) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = $1 AND "ipi" = $2"#)
            .bind(artist)
            .bind(ipi)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_artist_and_ipi_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, ipi_list: Vec<String>) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = ANY($1) AND "ipi" = ANY($2)"#)
            .bind(artist_list)
            .bind(ipi_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_artist_and_ipi_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, ipi: String) -> Result<Option<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE "artist" = $1 AND "ipi" = $2"#)
            .bind(artist)
            .bind(ipi)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistIpi>> {
        query_as::<_, ArtistIpi>(r#"SELECT * FROM "musicbrainz"."artist_ipi" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_ipi: ArtistIpi) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"INSERT INTO "artist_ipi" ("artist", "ipi", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(artist_ipi.edits_pending)
            .bind(artist_ipi.artist)
            .bind(artist_ipi.created)
            .bind(artist_ipi.ipi)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_ipi: ArtistIpi) -> Result<ArtistIpi> {
        query_as::<_, ArtistIpi>(r#"UPDATE "artist_ipi" SET "edits_pending" = $3, "created" = $4 WHERE "artist" = 1 AND "ipi" = 2 RETURNING *;"#)
            .bind(artist_ipi.artist)
            .bind(artist_ipi.ipi)
            .bind(artist_ipi.created)
            .bind(artist_ipi.edits_pending)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_ipi" WHERE "ipi" = 2 AND "artist" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
