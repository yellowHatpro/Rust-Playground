#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Mood;

pub struct MoodSet;

impl MoodSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, mood: Mood) -> Result<Mood> {
        query_as::<_, Mood>(r#"INSERT INTO "mood" ("id", "gid", "name", "comment", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(mood.name)
            .bind(mood.last_updated)
            .bind(mood.gid)
            .bind(mood.edits_pending)
            .bind(mood.id)
            .bind(mood.comment)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, mood: Mood) -> Result<Mood> {
        query_as::<_, Mood>(r#"UPDATE "mood" SET "last_updated" = $6, "comment" = $4, "gid" = $2, "name" = $3, "edits_pending" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(mood.id)
            .bind(mood.edits_pending)
            .bind(mood.last_updated)
            .bind(mood.name)
            .bind(mood.gid)
            .bind(mood.comment)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."mood" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
