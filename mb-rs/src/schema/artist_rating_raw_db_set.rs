#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistRatingRaw;

pub struct ArtistRatingRawSet;

impl ArtistRatingRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, editor: i32) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "artist" = $1 AND "editor" = $2"#)
            .bind(artist)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_artist_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "artist" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(artist_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist: i32, editor: i32) -> Result<Option<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "artist" = $1 AND "editor" = $2"#)
            .bind(artist)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<ArtistRatingRaw>> {
        query_as::<_, ArtistRatingRaw>(r#"SELECT * FROM "musicbrainz"."artist_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_rating_raw: ArtistRatingRaw) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"INSERT INTO "artist_rating_raw" ("artist", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(artist_rating_raw.artist)
            .bind(artist_rating_raw.editor)
            .bind(artist_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_rating_raw: ArtistRatingRaw) -> Result<ArtistRatingRaw> {
        query_as::<_, ArtistRatingRaw>(r#"UPDATE "artist_rating_raw" SET "rating" = $3 WHERE "artist" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(artist_rating_raw.artist)
            .bind(artist_rating_raw.editor)
            .bind(artist_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_rating_raw" WHERE "artist" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
