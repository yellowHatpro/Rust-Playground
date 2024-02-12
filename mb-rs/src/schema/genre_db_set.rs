#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Genre;

pub struct GenreSet;

impl GenreSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Genre> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Genre> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Genre> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Genre> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Genre> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Genre>> {
        query_as::<_, Genre>(r#"SELECT * FROM "musicbrainz"."genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, genre: Genre) -> Result<Genre> {
        query_as::<_, Genre>(r#"INSERT INTO "genre" ("id", "gid", "name", "comment", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(genre.id)
            .bind(genre.gid)
            .bind(genre.name)
            .bind(genre.comment)
            .bind(genre.edits_pending)
            .bind(genre.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, genre: Genre) -> Result<Genre> {
        query_as::<_, Genre>(r#"UPDATE "genre" SET "gid" = $2, "name" = $3, "comment" = $4, "edits_pending" = $5, "last_updated" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(genre.id)
            .bind(genre.gid)
            .bind(genre.name)
            .bind(genre.comment)
            .bind(genre.edits_pending)
            .bind(genre.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."genre" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
