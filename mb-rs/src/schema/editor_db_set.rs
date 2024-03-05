#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Editor;

pub struct EditorSet;

impl EditorSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_gender_id_where_gender_is<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE gender = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor: Editor) -> Result<Editor> {
        query_as::<_, Editor>(r#"INSERT INTO "editor" ("id", "name", "privs", "email", "website", "bio", "member_since", "email_confirm_date", "last_login_date", "last_updated", "birth_date", "gender", "area", "password", "ha1", "deleted") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(editor.name)
            .bind(editor.id)
            .bind(editor.website)
            .bind(editor.member_since)
            .bind(editor.last_updated)
            .bind(editor.privs)
            .bind(editor.last_login_date)
            .bind(editor.birth_date)
            .bind(editor.gender)
            .bind(editor.email_confirm_date)
            .bind(editor.bio)
            .bind(editor.ha1)
            .bind(editor.password)
            .bind(editor.email)
            .bind(editor.area)
            .bind(editor.deleted)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor: Editor) -> Result<Editor> {
        query_as::<_, Editor>(r#"UPDATE "editor" SET "name" = $2, "password" = $14, "website" = $5, "birth_date" = $11, "last_updated" = $10, "privs" = $3, "email_confirm_date" = $8, "area" = $13, "last_login_date" = $9, "email" = $4, "ha1" = $15, "deleted" = $16, "bio" = $6, "member_since" = $7, "gender" = $12 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor.gender)
            .bind(editor.ha1)
            .bind(editor.birth_date)
            .bind(editor.bio)
            .bind(editor.email_confirm_date)
            .bind(editor.email)
            .bind(editor.area)
            .bind(editor.id)
            .bind(editor.password)
            .bind(editor.deleted)
            .bind(editor.privs)
            .bind(editor.member_since)
            .bind(editor.last_login_date)
            .bind(editor.last_updated)
            .bind(editor.website)
            .bind(editor.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
