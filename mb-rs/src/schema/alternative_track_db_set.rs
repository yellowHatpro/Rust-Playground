#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeTrack;

pub struct AlternativeTrackSet;

impl AlternativeTrackSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_credit_id_where_artist_credit_is<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_track: AlternativeTrack) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"INSERT INTO "alternative_track" ("id", "name", "artist_credit", "ref_count") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(alternative_track.name)
            .bind(alternative_track.ref_count)
            .bind(alternative_track.artist_credit)
            .bind(alternative_track.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_track: AlternativeTrack) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"UPDATE "alternative_track" SET "artist_credit" = $3, "name" = $2, "ref_count" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(alternative_track.ref_count)
            .bind(alternative_track.artist_credit)
            .bind(alternative_track.id)
            .bind(alternative_track.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_track" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
