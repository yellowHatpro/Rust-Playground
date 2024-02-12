#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Editor;

pub struct EditorSet;

impl EditorSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Editor> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_gender_id<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE gender = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Editor>> {
        query_as::<_, Editor>(r#"SELECT * FROM "musicbrainz"."editor" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor: Editor) -> Result<Editor> {
        query_as::<_, Editor>(r#"INSERT INTO "editor" ("id", "name", "privs", "email", "website", "bio", "member_since", "email_confirm_date", "last_login_date", "last_updated", "birth_date", "gender", "area", "password", "ha1", "deleted") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(editor.id)
            .bind(editor.name)
            .bind(editor.privs)
            .bind(editor.email)
            .bind(editor.website)
            .bind(editor.bio)
            .bind(editor.member_since)
            .bind(editor.email_confirm_date)
            .bind(editor.last_login_date)
            .bind(editor.last_updated)
            .bind(editor.birth_date)
            .bind(editor.gender)
            .bind(editor.area)
            .bind(editor.password)
            .bind(editor.ha1)
            .bind(editor.deleted)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor: Editor) -> Result<Editor> {
        query_as::<_, Editor>(r#"UPDATE "editor" SET "name" = $2, "privs" = $3, "email" = $4, "website" = $5, "bio" = $6, "member_since" = $7, "email_confirm_date" = $8, "last_login_date" = $9, "last_updated" = $10, "birth_date" = $11, "gender" = $12, "area" = $13, "password" = $14, "ha1" = $15, "deleted" = $16 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor.id)
            .bind(editor.name)
            .bind(editor.privs)
            .bind(editor.email)
            .bind(editor.website)
            .bind(editor.bio)
            .bind(editor.member_since)
            .bind(editor.email_confirm_date)
            .bind(editor.last_login_date)
            .bind(editor.last_updated)
            .bind(editor.birth_date)
            .bind(editor.gender)
            .bind(editor.area)
            .bind(editor.password)
            .bind(editor.ha1)
            .bind(editor.deleted)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
