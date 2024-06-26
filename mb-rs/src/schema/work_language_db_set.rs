#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkLanguage;

pub struct WorkLanguageSet;

impl WorkLanguageSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkLanguage>> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_work_and_language<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, language: i32) -> Result<WorkLanguage> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language" WHERE "work" = $1 AND "language" = $2"#)
            .bind(work)
            .bind(language)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_work_and_language_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, language_list: Vec<i32>) -> Result<Vec<WorkLanguage>> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language" WHERE "work" = ANY($1) AND "language" = ANY($2)"#)
            .bind(work_list)
            .bind(language_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_work_and_language_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, language: i32) -> Result<Option<WorkLanguage>> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language" WHERE "work" = $1 AND "language" = $2"#)
            .bind(work)
            .bind(language)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkLanguage>> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_language_id_where_language_is<'e, E: PgExecutor<'e>>(executor: E, language_id: i32) -> Result<Vec<WorkLanguage>> {
        query_as::<_, WorkLanguage>(r#"SELECT * FROM "musicbrainz"."work_language" WHERE language = $1"#)
            .bind(language_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_language: WorkLanguage) -> Result<WorkLanguage> {
        query_as::<_, WorkLanguage>(r#"INSERT INTO "work_language" ("work", "language", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(work_language.language)
            .bind(work_language.work)
            .bind(work_language.created)
            .bind(work_language.edits_pending)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_language: WorkLanguage) -> Result<WorkLanguage> {
        query_as::<_, WorkLanguage>(r#"UPDATE "work_language" SET "edits_pending" = $3, "created" = $4 WHERE "language" = 2 AND "work" = 1 RETURNING *;"#)
            .bind(work_language.edits_pending)
            .bind(work_language.created)
            .bind(work_language.language)
            .bind(work_language.work)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_language" WHERE "work" = 1 AND "language" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
