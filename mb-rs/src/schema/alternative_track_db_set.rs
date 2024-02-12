#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeTrack;

pub struct AlternativeTrackSet;

impl AlternativeTrackSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<AlternativeTrack>> {
        query_as::<_, AlternativeTrack>(r#"SELECT * FROM "musicbrainz"."alternative_track" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_track: AlternativeTrack) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"INSERT INTO "alternative_track" ("id", "name", "artist_credit", "ref_count") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(alternative_track.id)
            .bind(alternative_track.name)
            .bind(alternative_track.artist_credit)
            .bind(alternative_track.ref_count)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_track: AlternativeTrack) -> Result<AlternativeTrack> {
        query_as::<_, AlternativeTrack>(r#"UPDATE "alternative_track" SET "name" = $2, "artist_credit" = $3, "ref_count" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(alternative_track.id)
            .bind(alternative_track.name)
            .bind(alternative_track.artist_credit)
            .bind(alternative_track.ref_count)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_track" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
