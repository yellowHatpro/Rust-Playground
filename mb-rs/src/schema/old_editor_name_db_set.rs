#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::OldEditorName;

pub struct OldEditorNameSet;

impl OldEditorNameSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<OldEditorName>> {
        query_as::<_, OldEditorName>(r#"SELECT * FROM "musicbrainz"."old_editor_name""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<OldEditorName>> {
        query_as::<_, OldEditorName>(r#"SELECT * FROM "musicbrainz"."old_editor_name" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, old_editor_name: OldEditorName) -> Result<OldEditorName> {
        query_as::<_, OldEditorName>(r#"INSERT INTO "old_editor_name" ("name") VALUES ($1) RETURNING *;"#)
            .bind(old_editor_name.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, old_editor_name: OldEditorName) -> Result<OldEditorName> {
        query_as::<_, OldEditorName>(r#"UPDATE "old_editor_name" SET "name" = $1 WHERE  RETURNING *;"#)
            .bind(old_editor_name.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."old_editor_name" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
