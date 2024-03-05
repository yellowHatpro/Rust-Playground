#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkType;

pub struct WorkTypeSet;

impl WorkTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkType>> {
        query_as::<_, WorkType>(r#"SELECT * FROM "musicbrainz"."work_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<WorkType> {
        query_as::<_, WorkType>(r#"SELECT * FROM "musicbrainz"."work_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<WorkType>> {
        query_as::<_, WorkType>(r#"SELECT * FROM "musicbrainz"."work_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<WorkType>> {
        query_as::<_, WorkType>(r#"SELECT * FROM "musicbrainz"."work_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, work_type_id: i32) -> Result<Vec<WorkType>> {
        query_as::<_, WorkType>(r#"SELECT * FROM "musicbrainz"."work_type" WHERE parent = $1"#)
            .bind(work_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_type: WorkType) -> Result<WorkType> {
        query_as::<_, WorkType>(r#"INSERT INTO "work_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(work_type.id)
            .bind(work_type.gid)
            .bind(work_type.parent)
            .bind(work_type.child_order)
            .bind(work_type.name)
            .bind(work_type.description)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_type: WorkType) -> Result<WorkType> {
        query_as::<_, WorkType>(r#"UPDATE "work_type" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(work_type.parent)
            .bind(work_type.name)
            .bind(work_type.gid)
            .bind(work_type.id)
            .bind(work_type.description)
            .bind(work_type.child_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
