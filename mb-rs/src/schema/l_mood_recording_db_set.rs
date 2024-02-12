#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LMoodRecording;

pub struct LMoodRecordingSet;

impl LMoodRecordingSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_id<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_mood_id<'e, E: PgExecutor<'e>>(executor: E, mood_id: i32) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE entity0 = $1"#)
            .bind(mood_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<LMoodRecording>> {
        query_as::<_, LMoodRecording>(r#"SELECT * FROM "musicbrainz"."l_mood_recording" WHERE entity1 = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_mood_recording: LMoodRecording) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"INSERT INTO "l_mood_recording" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_mood_recording.id)
            .bind(l_mood_recording.link)
            .bind(l_mood_recording.entity0)
            .bind(l_mood_recording.entity1)
            .bind(l_mood_recording.edits_pending)
            .bind(l_mood_recording.last_updated)
            .bind(l_mood_recording.link_order)
            .bind(l_mood_recording.entity0_credit)
            .bind(l_mood_recording.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_mood_recording: LMoodRecording) -> Result<LMoodRecording> {
        query_as::<_, LMoodRecording>(r#"UPDATE "l_mood_recording" SET "link" = $2, "entity0" = $3, "entity1" = $4, "edits_pending" = $5, "last_updated" = $6, "link_order" = $7, "entity0_credit" = $8, "entity1_credit" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_mood_recording.id)
            .bind(l_mood_recording.link)
            .bind(l_mood_recording.entity0)
            .bind(l_mood_recording.entity1)
            .bind(l_mood_recording.edits_pending)
            .bind(l_mood_recording.last_updated)
            .bind(l_mood_recording.link_order)
            .bind(l_mood_recording.entity0_credit)
            .bind(l_mood_recording.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_mood_recording" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
