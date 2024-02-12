#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Mood;

pub struct MoodSet;

impl MoodSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Mood> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Mood>> {
        query_as::<_, Mood>(r#"SELECT * FROM "musicbrainz"."mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, mood: Mood) -> Result<Mood> {
        query_as::<_, Mood>(r#"INSERT INTO "mood" ("id", "gid", "name", "comment", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(mood.id)
            .bind(mood.gid)
            .bind(mood.name)
            .bind(mood.comment)
            .bind(mood.edits_pending)
            .bind(mood.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, mood: Mood) -> Result<Mood> {
        query_as::<_, Mood>(r#"UPDATE "mood" SET "gid" = $2, "name" = $3, "comment" = $4, "edits_pending" = $5, "last_updated" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(mood.id)
            .bind(mood.gid)
            .bind(mood.name)
            .bind(mood.comment)
            .bind(mood.edits_pending)
            .bind(mood.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."mood" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
