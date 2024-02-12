#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::TrackRaw;

pub struct TrackRawSet;

impl TrackRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_raw_id<'e, E: PgExecutor<'e>>(executor: E, release_raw_id: i32) -> Result<Vec<TrackRaw>> {
        query_as::<_, TrackRaw>(r#"SELECT * FROM "musicbrainz"."track_raw" WHERE release = $1"#)
            .bind(release_raw_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, track_raw: TrackRaw) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"INSERT INTO "track_raw" ("id", "release", "title", "artist", "sequence") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(track_raw.id)
            .bind(track_raw.release)
            .bind(track_raw.title)
            .bind(track_raw.artist)
            .bind(track_raw.sequence)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, track_raw: TrackRaw) -> Result<TrackRaw> {
        query_as::<_, TrackRaw>(r#"UPDATE "track_raw" SET "release" = $2, "title" = $3, "artist" = $4, "sequence" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(track_raw.id)
            .bind(track_raw.release)
            .bind(track_raw.title)
            .bind(track_raw.artist)
            .bind(track_raw.sequence)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."track_raw" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
