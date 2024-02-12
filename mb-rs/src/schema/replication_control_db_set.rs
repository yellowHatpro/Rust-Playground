#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReplicationControl;

pub struct ReplicationControlSet;

impl ReplicationControlSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, replication_control: ReplicationControl) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"INSERT INTO "replication_control" ("id", "current_schema_sequence", "current_replication_sequence", "last_replication_date") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(replication_control.id)
            .bind(replication_control.current_schema_sequence)
            .bind(replication_control.current_replication_sequence)
            .bind(replication_control.last_replication_date)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, replication_control: ReplicationControl) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"UPDATE "replication_control" SET "current_schema_sequence" = $2, "current_replication_sequence" = $3, "last_replication_date" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(replication_control.id)
            .bind(replication_control.current_schema_sequence)
            .bind(replication_control.current_replication_sequence)
            .bind(replication_control.last_replication_date)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."replication_control" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
