#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Url;

pub struct UrlSet;

impl UrlSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Url> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Url>> {
        query_as::<_, Url>(r#"SELECT * FROM "musicbrainz"."url" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, url: Url) -> Result<Url> {
        query_as::<_, Url>(r#"INSERT INTO "url" ("id", "gid", "url", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(url.id)
            .bind(url.gid)
            .bind(url.url)
            .bind(url.edits_pending)
            .bind(url.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, url: Url) -> Result<Url> {
        query_as::<_, Url>(r#"UPDATE "url" SET "gid" = $2, "url" = $3, "edits_pending" = $4, "last_updated" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(url.id)
            .bind(url.gid)
            .bind(url.url)
            .bind(url.edits_pending)
            .bind(url.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."url" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
