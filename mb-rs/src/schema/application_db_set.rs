#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Application;

pub struct ApplicationSet;

impl ApplicationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Application> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_owner_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Application>> {
        query_as::<_, Application>(r#"SELECT * FROM "musicbrainz"."application" WHERE owner = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, application: Application) -> Result<Application> {
        query_as::<_, Application>(r#"INSERT INTO "application" ("id", "owner", "name", "oauth_id", "oauth_secret", "oauth_redirect_uri") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(application.id)
            .bind(application.owner)
            .bind(application.oauth_secret)
            .bind(application.oauth_id)
            .bind(application.oauth_redirect_uri)
            .bind(application.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, application: Application) -> Result<Application> {
        query_as::<_, Application>(r#"UPDATE "application" SET "oauth_redirect_uri" = $6, "owner" = $2, "oauth_id" = $4, "oauth_secret" = $5, "name" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(application.oauth_redirect_uri)
            .bind(application.name)
            .bind(application.owner)
            .bind(application.oauth_secret)
            .bind(application.oauth_id)
            .bind(application.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."application" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
