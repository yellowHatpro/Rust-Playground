#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkRatingRaw;

pub struct WorkRatingRawSet;

impl WorkRatingRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, editor: i32) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "work" = $1 AND "editor" = $2"#)
            .bind(work)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_work_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "work" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(work_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, editor: i32) -> Result<Option<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "work" = $1 AND "editor" = $2"#)
            .bind(work)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<WorkRatingRaw>> {
        query_as::<_, WorkRatingRaw>(r#"SELECT * FROM "musicbrainz"."work_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_rating_raw: WorkRatingRaw) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"INSERT INTO "work_rating_raw" ("work", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(work_rating_raw.work)
            .bind(work_rating_raw.editor)
            .bind(work_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_rating_raw: WorkRatingRaw) -> Result<WorkRatingRaw> {
        query_as::<_, WorkRatingRaw>(r#"UPDATE "work_rating_raw" SET "rating" = $3 WHERE "work" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(work_rating_raw.work)
            .bind(work_rating_raw.editor)
            .bind(work_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_rating_raw" WHERE "work" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
