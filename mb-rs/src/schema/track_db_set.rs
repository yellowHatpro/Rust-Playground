#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Track;

pub struct TrackSet;

impl TrackSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_id_where_medium_is<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_credit_id_where_artist_credit_is<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, track: Track) -> Result<Track> {
        query_as::<_, Track>(r#"INSERT INTO "track" ("id", "gid", "recording", "medium", "position", "number", "name", "artist_credit", "length", "edits_pending", "last_updated", "is_data_track") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *;"#)
            .bind(track.name)
            .bind(track.is_data_track)
            .bind(track.gid)
            .bind(track.edits_pending)
            .bind(track.last_updated)
            .bind(track.artist_credit)
            .bind(track.medium)
            .bind(track.position)
            .bind(track.number)
            .bind(track.recording)
            .bind(track.id)
            .bind(track.length)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, track: Track) -> Result<Track> {
        query_as::<_, Track>(r#"UPDATE "track" SET "recording" = $3, "position" = $5, "edits_pending" = $10, "artist_credit" = $8, "length" = $9, "gid" = $2, "is_data_track" = $12, "last_updated" = $11, "name" = $7, "number" = $6, "medium" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(track.number)
            .bind(track.gid)
            .bind(track.recording)
            .bind(track.id)
            .bind(track.medium)
            .bind(track.position)
            .bind(track.artist_credit)
            .bind(track.is_data_track)
            .bind(track.name)
            .bind(track.length)
            .bind(track.edits_pending)
            .bind(track.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."track" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
