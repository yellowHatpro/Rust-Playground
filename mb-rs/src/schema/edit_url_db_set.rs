#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditUrl;

pub struct EditUrlSet;

impl EditUrlSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_url<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, url: i32) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = $1 AND "url" = $2"#)
            .bind(edit)
            .bind(url)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_url_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, url_list: Vec<i32>) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = ANY($1) AND "url" = ANY($2)"#)
            .bind(edit_list)
            .bind(url_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_url_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, url: i32) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = $1 AND "url" = $2"#)
            .bind(edit)
            .bind(url)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_url_id_where_url_is<'e, E: PgExecutor<'e>>(executor: E, url_id: i32) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE url = $1"#)
            .bind(url_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_url: EditUrl) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"INSERT INTO "edit_url" ("edit", "url") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_url.url)
            .bind(edit_url.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_url: EditUrl) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"UPDATE "edit_url" SET  WHERE "edit" = 1 AND "url" = 2 RETURNING *;"#)
            .bind(edit_url.edit)
            .bind(edit_url.url)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_url" WHERE "edit" = 1 AND "url" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
