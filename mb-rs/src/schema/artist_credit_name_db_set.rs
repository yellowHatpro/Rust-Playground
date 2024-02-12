#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistCreditName;

pub struct ArtistCreditNameSet;

impl ArtistCreditNameSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_credit_and_position<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: i32, position: i16) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = $1 AND "position" = $2"#)
            .bind(artist_credit)
            .bind(position)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_artist_credit_and_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_list: Vec<i32>, position_list: Vec<i16>) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = ANY($1) AND "position" = ANY($2)"#)
            .bind(artist_credit_list)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_artist_credit_and_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: i32, position: i16) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = $1 AND "position" = $2"#)
            .bind(artist_credit)
            .bind(position)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_name: ArtistCreditName) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"INSERT INTO "artist_credit_name" ("artist_credit", "position", "artist", "name", "join_phrase") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(artist_credit_name.artist_credit)
            .bind(artist_credit_name.position)
            .bind(artist_credit_name.artist)
            .bind(artist_credit_name.name)
            .bind(artist_credit_name.join_phrase)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_name: ArtistCreditName) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"UPDATE "artist_credit_name" SET "artist" = $3, "name" = $4, "join_phrase" = $5 WHERE "artist_credit" = 1 AND "position" = 2 RETURNING *;"#)
            .bind(artist_credit_name.artist_credit)
            .bind(artist_credit_name.position)
            .bind(artist_credit_name.artist)
            .bind(artist_credit_name.name)
            .bind(artist_credit_name.join_phrase)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = 1 AND "position" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
