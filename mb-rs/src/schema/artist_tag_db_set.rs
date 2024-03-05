#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistTag;

pub struct ArtistTagSet;

impl ArtistTagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistTag>> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_artist_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, tag: i32) -> Result<ArtistTag> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag" WHERE "artist" = $1 AND "tag" = $2"#)
            .bind(artist)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_artist_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<ArtistTag>> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag" WHERE "artist" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(artist_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_artist_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, tag: i32) -> Result<Option<ArtistTag>> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag" WHERE "artist" = $1 AND "tag" = $2"#)
            .bind(artist)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistTag>> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<ArtistTag>> {
        query_as::<_, ArtistTag>(r#"SELECT * FROM "musicbrainz"."artist_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_tag: ArtistTag) -> Result<ArtistTag> {
        query_as::<_, ArtistTag>(r#"INSERT INTO "artist_tag" ("artist", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(artist_tag.tag)
            .bind(artist_tag.artist)
            .bind(artist_tag.last_updated)
            .bind(artist_tag.count)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_tag: ArtistTag) -> Result<ArtistTag> {
        query_as::<_, ArtistTag>(r#"UPDATE "artist_tag" SET "count" = $3, "last_updated" = $4 WHERE "tag" = 2 AND "artist" = 1 RETURNING *;"#)
            .bind(artist_tag.tag)
            .bind(artist_tag.last_updated)
            .bind(artist_tag.artist)
            .bind(artist_tag.count)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_tag" WHERE "artist" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
