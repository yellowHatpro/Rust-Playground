#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistIsni;

pub struct ArtistIsniSet;

impl ArtistIsniSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistIsni>> {
        query_as::<_, ArtistIsni>(r#"SELECT * FROM "musicbrainz"."artist_isni""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_artist_and_isni<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, isni: String) -> Result<ArtistIsni> {
        query_as::<_, ArtistIsni>(r#"SELECT * FROM "musicbrainz"."artist_isni" WHERE "artist" = $1 AND "isni" = $2"#)
            .bind(artist)
            .bind(isni)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_artist_and_isni_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, isni_list: Vec<String>) -> Result<Vec<ArtistIsni>> {
        query_as::<_, ArtistIsni>(r#"SELECT * FROM "musicbrainz"."artist_isni" WHERE "artist" = ANY($1) AND "isni" = ANY($2)"#)
            .bind(artist_list)
            .bind(isni_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_artist_and_isni_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, isni: String) -> Result<Option<ArtistIsni>> {
        query_as::<_, ArtistIsni>(r#"SELECT * FROM "musicbrainz"."artist_isni" WHERE "artist" = $1 AND "isni" = $2"#)
            .bind(artist)
            .bind(isni)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistIsni>> {
        query_as::<_, ArtistIsni>(r#"SELECT * FROM "musicbrainz"."artist_isni" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_isni: ArtistIsni) -> Result<ArtistIsni> {
        query_as::<_, ArtistIsni>(r#"INSERT INTO "artist_isni" ("artist", "isni", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(artist_isni.isni)
            .bind(artist_isni.edits_pending)
            .bind(artist_isni.artist)
            .bind(artist_isni.created)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_isni: ArtistIsni) -> Result<ArtistIsni> {
        query_as::<_, ArtistIsni>(r#"UPDATE "artist_isni" SET "edits_pending" = $3, "created" = $4 WHERE "isni" = 2 AND "artist" = 1 RETURNING *;"#)
            .bind(artist_isni.artist)
            .bind(artist_isni.edits_pending)
            .bind(artist_isni.created)
            .bind(artist_isni.isni)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_isni" WHERE "artist" = 1 AND "isni" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
