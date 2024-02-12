#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorOauthToken;

pub struct EditorOauthTokenSet;

impl EditorOauthTokenSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_application_id<'e, E: PgExecutor<'e>>(executor: E, application_id: i32) -> Result<Vec<EditorOauthToken>> {
        query_as::<_, EditorOauthToken>(r#"SELECT * FROM "musicbrainz"."editor_oauth_token" WHERE application = $1"#)
            .bind(application_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_oauth_token: EditorOauthToken) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"INSERT INTO "editor_oauth_token" ("id", "editor", "application", "authorization_code", "refresh_token", "access_token", "expire_time", "scope", "granted", "code_challenge", "code_challenge_method") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *;"#)
            .bind(editor_oauth_token.id)
            .bind(editor_oauth_token.editor)
            .bind(editor_oauth_token.application)
            .bind(editor_oauth_token.authorization_code)
            .bind(editor_oauth_token.refresh_token)
            .bind(editor_oauth_token.access_token)
            .bind(editor_oauth_token.expire_time)
            .bind(editor_oauth_token.scope)
            .bind(editor_oauth_token.granted)
            .bind(editor_oauth_token.code_challenge)
            .bind(editor_oauth_token.code_challenge_method)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_oauth_token: EditorOauthToken) -> Result<EditorOauthToken> {
        query_as::<_, EditorOauthToken>(r#"UPDATE "editor_oauth_token" SET "editor" = $2, "application" = $3, "authorization_code" = $4, "refresh_token" = $5, "access_token" = $6, "expire_time" = $7, "scope" = $8, "granted" = $9, "code_challenge" = $10, "code_challenge_method" = $11 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_oauth_token.id)
            .bind(editor_oauth_token.editor)
            .bind(editor_oauth_token.application)
            .bind(editor_oauth_token.authorization_code)
            .bind(editor_oauth_token.refresh_token)
            .bind(editor_oauth_token.access_token)
            .bind(editor_oauth_token.expire_time)
            .bind(editor_oauth_token.scope)
            .bind(editor_oauth_token.granted)
            .bind(editor_oauth_token.code_challenge)
            .bind(editor_oauth_token.code_challenge_method)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_oauth_token" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
