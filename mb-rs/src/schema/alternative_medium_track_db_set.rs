#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeMediumTrack;

pub struct AlternativeMediumTrackSet;

impl AlternativeMediumTrackSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_alternative_medium_and_track<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: i32, track: i32) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = $1 AND "track" = $2"#)
            .bind(alternative_medium)
            .bind(track)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_alternative_medium_and_track_list<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_list: Vec<i32>, track_list: Vec<i32>) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = ANY($1) AND "track" = ANY($2)"#)
            .bind(alternative_medium_list)
            .bind(track_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_alternative_medium_and_track_optional<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: i32, track: i32) -> Result<Option<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE "alternative_medium" = $1 AND "track" = $2"#)
            .bind(alternative_medium)
            .bind(track)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_alternative_medium_id_where_alternative_medium_is<'e, E: PgExecutor<'e>>(executor: E, alternative_medium_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE alternative_medium = $1"#)
            .bind(alternative_medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_track_id_where_track_is<'e, E: PgExecutor<'e>>(executor: E, track_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE track = $1"#)
            .bind(track_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_alternative_track_id_where_alternative_track_is<'e, E: PgExecutor<'e>>(executor: E, alternative_track_id: i32) -> Result<Vec<AlternativeMediumTrack>> {
        query_as::<_, AlternativeMediumTrack>(r#"SELECT * FROM "musicbrainz"."alternative_medium_track" WHERE alternative_track = $1"#)
            .bind(alternative_track_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_track: AlternativeMediumTrack) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"INSERT INTO "alternative_medium_track" ("alternative_medium", "track", "alternative_track") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(alternative_medium_track.alternative_track)
            .bind(alternative_medium_track.alternative_medium)
            .bind(alternative_medium_track.track)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium_track: AlternativeMediumTrack) -> Result<AlternativeMediumTrack> {
        query_as::<_, AlternativeMediumTrack>(r#"UPDATE "alternative_medium_track" SET "alternative_track" = $3 WHERE "alternative_medium" = 1 AND "track" = 2 RETURNING *;"#)
            .bind(alternative_medium_track.track)
            .bind(alternative_medium_track.alternative_track)
            .bind(alternative_medium_track.alternative_medium)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_medium_track" WHERE "track" = 2 AND "alternative_medium" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
