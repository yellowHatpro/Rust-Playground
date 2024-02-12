#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Script;

pub struct ScriptSet;

impl ScriptSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Script> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Script> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Script> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Script> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Script> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Script>> {
        query_as::<_, Script>(r#"SELECT * FROM "musicbrainz"."script" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, script: Script) -> Result<Script> {
        query_as::<_, Script>(r#"INSERT INTO "script" ("id", "iso_code", "iso_number", "name", "frequency") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(script.id)
            .bind(script.iso_code)
            .bind(script.iso_number)
            .bind(script.name)
            .bind(script.frequency)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, script: Script) -> Result<Script> {
        query_as::<_, Script>(r#"UPDATE "script" SET "iso_code" = $2, "iso_number" = $3, "name" = $4, "frequency" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(script.id)
            .bind(script.iso_code)
            .bind(script.iso_number)
            .bind(script.name)
            .bind(script.frequency)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."script" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
