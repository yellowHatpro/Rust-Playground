#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditUrl;

pub struct EditUrlSet;

impl EditUrlSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_url<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, url: i32) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = $1 AND "url" = $2"#)
            .bind(edit)
            .bind(url)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_url_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, url_list: Vec<i32>) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = ANY($1) AND "url" = ANY($2)"#)
            .bind(edit_list)
            .bind(url_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_url_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, url: i32) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "edit" = $1 AND "url" = $2"#)
            .bind(edit)
            .bind(url)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_url_id<'e, E: PgExecutor<'e>>(executor: E, url_id: i32) -> Result<Vec<EditUrl>> {
        query_as::<_, EditUrl>(r#"SELECT * FROM "musicbrainz"."edit_url" WHERE url = $1"#)
            .bind(url_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_url: EditUrl) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"INSERT INTO "edit_url" ("edit", "url") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_url.edit)
            .bind(edit_url.url)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_url: EditUrl) -> Result<EditUrl> {
        query_as::<_, EditUrl>(r#"UPDATE "edit_url" SET  WHERE "edit" = 1 AND "url" = 2 RETURNING *;"#)
            .bind(edit_url.edit)
            .bind(edit_url.url)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_url" WHERE "edit" = 1 AND "url" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
