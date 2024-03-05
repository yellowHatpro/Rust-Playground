#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistCreditName;

pub struct ArtistCreditNameSet;

impl ArtistCreditNameSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_artist_credit_and_position<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: i32, position: i16) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = $1 AND "position" = $2"#)
            .bind(artist_credit)
            .bind(position)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_artist_credit_and_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_list: Vec<i32>, position_list: Vec<i16>) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = ANY($1) AND "position" = ANY($2)"#)
            .bind(artist_credit_list)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_artist_credit_and_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: i32, position: i16) -> Result<Option<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = $1 AND "position" = $2"#)
            .bind(artist_credit)
            .bind(position)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_credit_id_where_artist_credit_is<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id_where_artist_is<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistCreditName>> {
        query_as::<_, ArtistCreditName>(r#"SELECT * FROM "musicbrainz"."artist_credit_name" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_name: ArtistCreditName) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"INSERT INTO "artist_credit_name" ("artist_credit", "position", "artist", "name", "join_phrase") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(artist_credit_name.artist)
            .bind(artist_credit_name.artist_credit)
            .bind(artist_credit_name.position)
            .bind(artist_credit_name.name)
            .bind(artist_credit_name.join_phrase)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit_name: ArtistCreditName) -> Result<ArtistCreditName> {
        query_as::<_, ArtistCreditName>(r#"UPDATE "artist_credit_name" SET "name" = $4, "join_phrase" = $5, "artist" = $3 WHERE "position" = 2 AND "artist_credit" = 1 RETURNING *;"#)
            .bind(artist_credit_name.name)
            .bind(artist_credit_name.artist)
            .bind(artist_credit_name.artist_credit)
            .bind(artist_credit_name.join_phrase)
            .bind(artist_credit_name.position)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_credit_name" WHERE "artist_credit" = 1 AND "position" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
