#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::DeletedEntity;

pub struct DeletedEntitySet;

impl DeletedEntitySet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, gid_list: Vec<uuid::Uuid>) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = ANY($1)"#)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<DeletedEntity>> {
        query_as::<_, DeletedEntity>(r#"SELECT * FROM "musicbrainz"."deleted_entity" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, deleted_entity: DeletedEntity) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"INSERT INTO "deleted_entity" ("gid", "data", "deleted_at") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(deleted_entity.gid)
            .bind(deleted_entity.data)
            .bind(deleted_entity.deleted_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, deleted_entity: DeletedEntity) -> Result<DeletedEntity> {
        query_as::<_, DeletedEntity>(r#"UPDATE "deleted_entity" SET "data" = $2, "deleted_at" = $3 WHERE "gid" = 1 RETURNING *;"#)
            .bind(deleted_entity.gid)
            .bind(deleted_entity.data)
            .bind(deleted_entity.deleted_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."deleted_entity" WHERE "gid" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
