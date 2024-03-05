#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Iswc;

pub struct IswcSet;

impl IswcSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Iswc>> {
        query_as::<_, Iswc>(r#"SELECT * FROM "musicbrainz"."iswc""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Iswc> {
        query_as::<_, Iswc>(r#"SELECT * FROM "musicbrainz"."iswc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Iswc>> {
        query_as::<_, Iswc>(r#"SELECT * FROM "musicbrainz"."iswc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Iswc>> {
        query_as::<_, Iswc>(r#"SELECT * FROM "musicbrainz"."iswc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<Iswc>> {
        query_as::<_, Iswc>(r#"SELECT * FROM "musicbrainz"."iswc" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, iswc: Iswc) -> Result<Iswc> {
        query_as::<_, Iswc>(r#"INSERT INTO "iswc" ("id", "work", "iswc", "source", "edits_pending", "created") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(iswc.source)
            .bind(iswc.id)
            .bind(iswc.work)
            .bind(iswc.edits_pending)
            .bind(iswc.created)
            .bind(iswc.iswc)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, iswc: Iswc) -> Result<Iswc> {
        query_as::<_, Iswc>(r#"UPDATE "iswc" SET "edits_pending" = $5, "work" = $2, "iswc" = $3, "created" = $6, "source" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(iswc.iswc)
            .bind(iswc.id)
            .bind(iswc.work)
            .bind(iswc.source)
            .bind(iswc.edits_pending)
            .bind(iswc.created)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."iswc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
