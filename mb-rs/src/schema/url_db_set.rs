#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Url;

pub struct UrlSet;

impl UrlSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, url: Url) -> Result<Url> {
        query_as::<_, Url>(r#"INSERT INTO "url" ("id", "gid", "url", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(url.url)
            .bind(url.edits_pending)
            .bind(url.last_updated)
            .bind(url.id)
            .bind(url.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, url: Url) -> Result<Url> {
        query_as::<_, Url>(r#"UPDATE "url" SET "gid" = $2, "last_updated" = $5, "url" = $3, "edits_pending" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(url.url)
            .bind(url.last_updated)
            .bind(url.id)
            .bind(url.gid)
            .bind(url.edits_pending)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."url" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
