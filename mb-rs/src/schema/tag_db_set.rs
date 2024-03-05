#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Tag;

pub struct TagSet;

impl TagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Tag>> {
        query_as::<_, Tag>(r#"SELECT * FROM "musicbrainz"."tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Tag> {
        query_as::<_, Tag>(r#"SELECT * FROM "musicbrainz"."tag" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Tag>> {
        query_as::<_, Tag>(r#"SELECT * FROM "musicbrainz"."tag" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Tag>> {
        query_as::<_, Tag>(r#"SELECT * FROM "musicbrainz"."tag" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, tag: Tag) -> Result<Tag> {
        query_as::<_, Tag>(r#"INSERT INTO "tag" ("id", "name", "ref_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(tag.id)
            .bind(tag.ref_count)
            .bind(tag.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, tag: Tag) -> Result<Tag> {
        query_as::<_, Tag>(r#"UPDATE "tag" SET "name" = $2, "ref_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(tag.ref_count)
            .bind(tag.id)
            .bind(tag.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."tag" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
