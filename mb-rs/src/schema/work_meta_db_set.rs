#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkMeta;

pub struct WorkMetaSet;

impl WorkMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkMeta>> {
        query_as::<_, WorkMeta>(r#"SELECT * FROM "musicbrainz"."work_meta" WHERE id = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_meta: WorkMeta) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"INSERT INTO "work_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(work_meta.rating)
            .bind(work_meta.rating_count)
            .bind(work_meta.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_meta: WorkMeta) -> Result<WorkMeta> {
        query_as::<_, WorkMeta>(r#"UPDATE "work_meta" SET "rating_count" = $3, "rating" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(work_meta.rating)
            .bind(work_meta.id)
            .bind(work_meta.rating_count)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
