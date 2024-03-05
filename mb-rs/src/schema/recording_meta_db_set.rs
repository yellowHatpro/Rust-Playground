#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingMeta;

pub struct RecordingMetaSet;

impl RecordingMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingMeta>> {
        query_as::<_, RecordingMeta>(r#"SELECT * FROM "musicbrainz"."recording_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<RecordingMeta> {
        query_as::<_, RecordingMeta>(r#"SELECT * FROM "musicbrainz"."recording_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<RecordingMeta>> {
        query_as::<_, RecordingMeta>(r#"SELECT * FROM "musicbrainz"."recording_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<RecordingMeta>> {
        query_as::<_, RecordingMeta>(r#"SELECT * FROM "musicbrainz"."recording_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingMeta>> {
        query_as::<_, RecordingMeta>(r#"SELECT * FROM "musicbrainz"."recording_meta" WHERE id = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_meta: RecordingMeta) -> Result<RecordingMeta> {
        query_as::<_, RecordingMeta>(r#"INSERT INTO "recording_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(recording_meta.rating)
            .bind(recording_meta.rating_count)
            .bind(recording_meta.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_meta: RecordingMeta) -> Result<RecordingMeta> {
        query_as::<_, RecordingMeta>(r#"UPDATE "recording_meta" SET "rating_count" = $3, "rating" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(recording_meta.rating)
            .bind(recording_meta.id)
            .bind(recording_meta.rating_count)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
