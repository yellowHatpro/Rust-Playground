#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkTag;

pub struct WorkTagSet;

impl WorkTagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_work_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, tag: i32) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = $1 AND "tag" = $2"#)
            .bind(work)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_work_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(work_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_work_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, tag: i32) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = $1 AND "tag" = $2"#)
            .bind(work)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag: WorkTag) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"INSERT INTO "work_tag" ("work", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(work_tag.work)
            .bind(work_tag.count)
            .bind(work_tag.tag)
            .bind(work_tag.last_updated)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag: WorkTag) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"UPDATE "work_tag" SET "count" = $3, "last_updated" = $4 WHERE "work" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(work_tag.last_updated)
            .bind(work_tag.work)
            .bind(work_tag.count)
            .bind(work_tag.tag)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_tag" WHERE "work" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
