#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DeletedEntity;

pub struct DeletedEntitySet;

impl DeletedEntitySet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_gid<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, gid_list: Vec<uuid::Uuid>) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = ANY($1)"#)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, deleted_entity: DeletedEntity) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"INSERT INTO "deleted_entity" ("gid", "data", "deleted_at") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(deleted_entity.gid)
            .bind(deleted_entity.data)
            .bind(deleted_entity.deleted_at)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, deleted_entity: DeletedEntity) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"UPDATE "deleted_entity" SET "deleted_at" = $3, "data" = $2 WHERE "gid" = 1 RETURNING *;"#)
            .bind(deleted_entity.data)
            .bind(deleted_entity.gid)
            .bind(deleted_entity.deleted_at)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."deleted_entity" WHERE "gid" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
