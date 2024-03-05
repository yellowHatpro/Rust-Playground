#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReplicationControl;

pub struct ReplicationControlSet;

impl ReplicationControlSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReplicationControl>> {
        query_as::<_, ReplicationControl>(r#"SELECT * FROM "musicbrainz"."replication_control" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, replication_control: ReplicationControl) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"INSERT INTO "replication_control" ("id", "current_schema_sequence", "current_replication_sequence", "last_replication_date") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(replication_control.id)
            .bind(replication_control.current_replication_sequence)
            .bind(replication_control.current_schema_sequence)
            .bind(replication_control.last_replication_date)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, replication_control: ReplicationControl) -> Result<ReplicationControl> {
        query_as::<_, ReplicationControl>(r#"UPDATE "replication_control" SET "last_replication_date" = $4, "current_schema_sequence" = $2, "current_replication_sequence" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(replication_control.current_replication_sequence)
            .bind(replication_control.current_schema_sequence)
            .bind(replication_control.id)
            .bind(replication_control.last_replication_date)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."replication_control" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
