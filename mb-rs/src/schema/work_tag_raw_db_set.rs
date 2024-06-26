#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkTagRaw;

pub struct WorkTagRawSet;

impl WorkTagRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_work_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, editor: i32, tag: i32) -> Result<WorkTagRaw> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE "work" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(work)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_work_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE "work" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(work_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_work_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, editor: i32, tag: i32) -> Result<Option<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE "work" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(work)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<WorkTagRaw>> {
        query_as::<_, WorkTagRaw>(r#"SELECT * FROM "musicbrainz"."work_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag_raw: WorkTagRaw) -> Result<WorkTagRaw> {
        query_as::<_, WorkTagRaw>(r#"INSERT INTO "work_tag_raw" ("work", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(work_tag_raw.is_upvote)
            .bind(work_tag_raw.tag)
            .bind(work_tag_raw.editor)
            .bind(work_tag_raw.work)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag_raw: WorkTagRaw) -> Result<WorkTagRaw> {
        query_as::<_, WorkTagRaw>(r#"UPDATE "work_tag_raw" SET "is_upvote" = $4 WHERE "work" = 1 AND "editor" = 2 AND "tag" = 3 RETURNING *;"#)
            .bind(work_tag_raw.editor)
            .bind(work_tag_raw.work)
            .bind(work_tag_raw.tag)
            .bind(work_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_tag_raw" WHERE "work" = 1 AND "editor" = 2 AND "tag" = 3"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
