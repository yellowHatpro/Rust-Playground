#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Track;

pub struct TrackSet;

impl TrackSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = $1"#)
            .bind(position)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, position_list: Vec<i32>) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = ANY($1)"#)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = $1"#)
            .bind(position)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, medium: i32) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "medium" = $1"#)
            .bind(medium)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, medium_list: Vec<i32>) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "medium" = ANY($1)"#)
            .bind(medium_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medium: i32) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "medium" = $1"#)
            .bind(medium)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Track> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = $1"#)
            .bind(position)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, position_list: Vec<i32>) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = ANY($1)"#)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Option<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE "position" = $1"#)
            .bind(position)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_id<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<Track>> {
        query_as::<_, Track>(r#"SELECT * FROM "musicbrainz"."track" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, track: Track) -> Result<Track> {
        query_as::<_, Track>(r#"INSERT INTO "track" ("id", "gid", "recording", "medium", "position", "number", "name", "artist_credit", "length", "edits_pending", "last_updated", "is_data_track") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *;"#)
            .bind(track.id)
            .bind(track.gid)
            .bind(track.recording)
            .bind(track.medium)
            .bind(track.position)
            .bind(track.number)
            .bind(track.name)
            .bind(track.artist_credit)
            .bind(track.length)
            .bind(track.edits_pending)
            .bind(track.last_updated)
            .bind(track.is_data_track)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, track: Track) -> Result<Track> {
        query_as::<_, Track>(r#"UPDATE "track" SET "gid" = $2, "recording" = $3, "medium" = $4, "position" = $5, "number" = $6, "name" = $7, "artist_credit" = $8, "length" = $9, "edits_pending" = $10, "last_updated" = $11, "is_data_track" = $12 WHERE "id" = 1 RETURNING *;"#)
            .bind(track.id)
            .bind(track.gid)
            .bind(track.recording)
            .bind(track.medium)
            .bind(track.position)
            .bind(track.number)
            .bind(track.name)
            .bind(track.artist_credit)
            .bind(track.length)
            .bind(track.edits_pending)
            .bind(track.last_updated)
            .bind(track.is_data_track)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."track" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
