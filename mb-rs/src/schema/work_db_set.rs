#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Work;

pub struct WorkSet;

impl WorkSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Work> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Work> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Work> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Work> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Work> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_work_type_id<'e, E: PgExecutor<'e>>(executor: E, work_type_id: i32) -> Result<Vec<Work>> {
        query_as::<_, Work>(r#"SELECT * FROM "musicbrainz"."work" WHERE type = $1"#)
            .bind(work_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work: Work) -> Result<Work> {
        query_as::<_, Work>(r#"INSERT INTO "work" ("id", "gid", "name", "type", "comment", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(work.id)
            .bind(work.gid)
            .bind(work.name)
            .bind(work.Type)
            .bind(work.comment)
            .bind(work.edits_pending)
            .bind(work.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work: Work) -> Result<Work> {
        query_as::<_, Work>(r#"UPDATE "work" SET "gid" = $2, "name" = $3, "type" = $4, "comment" = $5, "edits_pending" = $6, "last_updated" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(work.id)
            .bind(work.gid)
            .bind(work.name)
            .bind(work.Type)
            .bind(work.comment)
            .bind(work.edits_pending)
            .bind(work.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
