#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeMediumTrack;

pub struct AlternativeMediumTrackSet;

impl AlternativeMediumTrackSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_alternative_medium_and_track<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: i32, track: i32) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = $1 AND "track" = $2"#)
            .bind(alternative_medium)
            .bind(track)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_alternative_medium_and_track_list<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_list: Vec<i32>, track_list: Vec<i32>) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = ANY($1) AND "track" = ANY($2)"#)
            .bind(alternative_medium_list)
            .bind(track_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_alternative_medium_and_track_optional<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: i32, track: i32) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = $1 AND "track" = $2"#)
            .bind(alternative_medium)
            .bind(track)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_alternative_medium_id<'e, E: PgExecutor<'e>>(executor: E, alternative_medium_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE alternative_medium = $1"#)
            .bind(alternative_medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_track_id<'e, E: PgExecutor<'e>>(executor: E, track_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE track = $1"#)
            .bind(track_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_alternative_track_id<'e, E: PgExecutor<'e>>(executor: E, alternative_track_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE alternative_track = $1"#)
            .bind(alternative_track_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_track: AlternativeMediumTrack) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"INSERT INTO "alternative_medium_track" ("alternative_medium", "track", "alternative_track") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(alternative_medium_track.alternative_medium)
            .bind(alternative_medium_track.track)
            .bind(alternative_medium_track.alternative_track)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_track: AlternativeMediumTrack) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"UPDATE "alternative_medium_track" SET "alternative_track" = $3 WHERE "alternative_medium" = 1 AND "track" = 2 RETURNING *;"#)
            .bind(alternative_medium_track.alternative_medium)
            .bind(alternative_medium_track.track)
            .bind(alternative_medium_track.alternative_track)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = 1 AND "track" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
