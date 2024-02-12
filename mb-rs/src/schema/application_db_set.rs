#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Application;

pub struct ApplicationSet;

impl ApplicationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE owner = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, application: Application) -> Result<Application> {
        query_as::<_, Application>(r#"INSERT INTO "application" ("id", "owner", "name", "oauth_id", "oauth_secret", "oauth_redirect_uri") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(application.id)
            .bind(application.owner)
            .bind(application.name)
            .bind(application.oauth_id)
            .bind(application.oauth_secret)
            .bind(application.oauth_redirect_uri)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, application: Application) -> Result<Application> {
        query_as::<_, Application>(r#"UPDATE "application" SET "owner" = $2, "name" = $3, "oauth_id" = $4, "oauth_secret" = $5, "oauth_redirect_uri" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(application.id)
            .bind(application.owner)
            .bind(application.name)
            .bind(application.oauth_id)
            .bind(application.oauth_secret)
            .bind(application.oauth_redirect_uri)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."application" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
