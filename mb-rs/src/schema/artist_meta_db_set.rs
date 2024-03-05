#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistMeta;

pub struct ArtistMetaSet;

impl ArtistMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistMeta>> {
        query_as::<_, ArtistMeta>(r#"SELECT * FROM "musicbrainz"."artist_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistMeta> {
        query_as::<_, ArtistMeta>(r#"SELECT * FROM "musicbrainz"."artist_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistMeta>> {
        query_as::<_, ArtistMeta>(r#"SELECT * FROM "musicbrainz"."artist_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistMeta>> {
        query_as::<_, ArtistMeta>(r#"SELECT * FROM "musicbrainz"."artist_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistMeta>> {
        query_as::<_, ArtistMeta>(r#"SELECT * FROM "musicbrainz"."artist_meta" WHERE id = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_meta: ArtistMeta) -> Result<ArtistMeta> {
        query_as::<_, ArtistMeta>(r#"INSERT INTO "artist_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(artist_meta.rating)
            .bind(artist_meta.rating_count)
            .bind(artist_meta.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_meta: ArtistMeta) -> Result<ArtistMeta> {
        query_as::<_, ArtistMeta>(r#"UPDATE "artist_meta" SET "rating" = $2, "rating_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_meta.rating)
            .bind(artist_meta.id)
            .bind(artist_meta.rating_count)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
