#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkMeta;

pub struct WorkMetaSet;

impl WorkMetaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE id = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_meta: WorkMeta) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"INSERT INTO "work_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(work_meta.id)
            .bind(work_meta.rating)
            .bind(work_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_meta: WorkMeta) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"UPDATE "work_meta" SET "rating" = $2, "rating_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(work_meta.id)
            .bind(work_meta.rating)
            .bind(work_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
