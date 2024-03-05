#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeReleaseType;

pub struct AlternativeReleaseTypeSet;

impl AlternativeReleaseTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeReleaseType>> {
        query_as::<_, AlternativeReleaseType>(r#"SELECT * FROM "musicbrainz"."alternative_release_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AlternativeReleaseType> {
        query_as::<_, AlternativeReleaseType>(r#"SELECT * FROM "musicbrainz"."alternative_release_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AlternativeReleaseType>> {
        query_as::<_, AlternativeReleaseType>(r#"SELECT * FROM "musicbrainz"."alternative_release_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AlternativeReleaseType>> {
        query_as::<_, AlternativeReleaseType>(r#"SELECT * FROM "musicbrainz"."alternative_release_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_alternative_release_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, alternative_release_type_id: i32) -> Result<Vec<AlternativeReleaseType>> {
        query_as::<_, AlternativeReleaseType>(r#"SELECT * FROM "musicbrainz"."alternative_release_type" WHERE parent = $1"#)
            .bind(alternative_release_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_release_type: AlternativeReleaseType) -> Result<AlternativeReleaseType> {
        query_as::<_, AlternativeReleaseType>(r#"INSERT INTO "alternative_release_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(alternative_release_type.id)
            .bind(alternative_release_type.name)
            .bind(alternative_release_type.child_order)
            .bind(alternative_release_type.description)
            .bind(alternative_release_type.gid)
            .bind(alternative_release_type.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_release_type: AlternativeReleaseType) -> Result<AlternativeReleaseType> {
        query_as::<_, AlternativeReleaseType>(r#"UPDATE "alternative_release_type" SET "gid" = $6, "description" = $5, "child_order" = $4, "name" = $2, "parent" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(alternative_release_type.name)
            .bind(alternative_release_type.id)
            .bind(alternative_release_type.parent)
            .bind(alternative_release_type.gid)
            .bind(alternative_release_type.child_order)
            .bind(alternative_release_type.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_release_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
