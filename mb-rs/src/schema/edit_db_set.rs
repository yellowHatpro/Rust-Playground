#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Edit;

pub struct EditSet;

impl EditSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Edit>> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Edit> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Edit>> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Edit>> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Edit>> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_language_id_where_language_is<'e, E: PgExecutor<'e>>(executor: E, language_id: i32) -> Result<Vec<Edit>> {
        query_as::<_, Edit>(r#"SELECT * FROM "musicbrainz"."edit" WHERE language = $1"#)
            .bind(language_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit: Edit) -> Result<Edit> {
        query_as::<_, Edit>(r#"INSERT INTO "edit" ("id", "editor", "type", "status", "autoedit", "open_time", "close_time", "expire_time", "language", "quality") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;"#)
            .bind(edit.editor)
            .bind(edit.autoedit)
            .bind(edit.quality)
            .bind(edit.open_time)
            .bind(edit.close_time)
            .bind(edit.expire_time)
            .bind(edit.Type)
            .bind(edit.id)
            .bind(edit.language)
            .bind(edit.status)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit: Edit) -> Result<Edit> {
        query_as::<_, Edit>(r#"UPDATE "edit" SET "type" = $3, "open_time" = $6, "editor" = $2, "expire_time" = $8, "status" = $4, "close_time" = $7, "autoedit" = $5, "language" = $9, "quality" = $10 WHERE "id" = 1 RETURNING *;"#)
            .bind(edit.editor)
            .bind(edit.expire_time)
            .bind(edit.id)
            .bind(edit.language)
            .bind(edit.close_time)
            .bind(edit.quality)
            .bind(edit.autoedit)
            .bind(edit.Type)
            .bind(edit.open_time)
            .bind(edit.status)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
