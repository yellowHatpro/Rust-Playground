#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::UnreferencedRowLog;

pub struct UnreferencedRowLogSet;

impl UnreferencedRowLogSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_table_name_and_row_id<'e, E: PgExecutor<'e>>(&self, executor: E, table_name: String, row_id: i32) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "table_name" = $1 AND "row_id" = $2"#)
            .bind(table_name)
            .bind(row_id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_table_name_and_row_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, table_name_list: Vec<String>, row_id_list: Vec<i32>) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "table_name" = ANY($1) AND "row_id" = ANY($2)"#)
            .bind(table_name_list)
            .bind(row_id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_table_name_and_row_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, table_name: String, row_id: i32) -> Result<Option<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "table_name" = $1 AND "row_id" = $2"#)
            .bind(table_name)
            .bind(row_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<UnreferencedRowLog>> {
        query_as::<_, UnreferencedRowLog>(r#"SELECT * FROM "musicbrainz"."unreferenced_row_log" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, unreferenced_row_log: UnreferencedRowLog) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"INSERT INTO "unreferenced_row_log" ("table_name", "row_id", "inserted") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(unreferenced_row_log.table_name)
            .bind(unreferenced_row_log.row_id)
            .bind(unreferenced_row_log.inserted)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, unreferenced_row_log: UnreferencedRowLog) -> Result<UnreferencedRowLog> {
        query_as::<_, UnreferencedRowLog>(r#"UPDATE "unreferenced_row_log" SET "inserted" = $3 WHERE "table_name" = 1 AND "row_id" = 2 RETURNING *;"#)
            .bind(unreferenced_row_log.table_name)
            .bind(unreferenced_row_log.row_id)
            .bind(unreferenced_row_log.inserted)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."unreferenced_row_log" WHERE "table_name" = 1 AND "row_id" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
