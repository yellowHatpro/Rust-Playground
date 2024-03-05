#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LGenreRecording;

pub struct LGenreRecordingSet;

impl LGenreRecordingSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LGenreRecording> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE entity0 = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<LGenreRecording>> {
        query_as::<_, LGenreRecording>(r#"SELECT * FROM "musicbrainz"."l_genre_recording" WHERE entity1 = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_recording: LGenreRecording) -> Result<LGenreRecording> {
        query_as::<_, LGenreRecording>(r#"INSERT INTO "l_genre_recording" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_genre_recording.entity1_credit)
            .bind(l_genre_recording.entity0)
            .bind(l_genre_recording.id)
            .bind(l_genre_recording.edits_pending)
            .bind(l_genre_recording.last_updated)
            .bind(l_genre_recording.entity1)
            .bind(l_genre_recording.link_order)
            .bind(l_genre_recording.link)
            .bind(l_genre_recording.entity0_credit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_recording: LGenreRecording) -> Result<LGenreRecording> {
        query_as::<_, LGenreRecording>(r#"UPDATE "l_genre_recording" SET "entity0" = $3, "entity1" = $4, "edits_pending" = $5, "last_updated" = $6, "link" = $2, "entity0_credit" = $8, "entity1_credit" = $9, "link_order" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_genre_recording.edits_pending)
            .bind(l_genre_recording.id)
            .bind(l_genre_recording.entity0_credit)
            .bind(l_genre_recording.entity0)
            .bind(l_genre_recording.link)
            .bind(l_genre_recording.entity1_credit)
            .bind(l_genre_recording.last_updated)
            .bind(l_genre_recording.entity1)
            .bind(l_genre_recording.link_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_genre_recording" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
