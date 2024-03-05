#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupPrimaryType;

pub struct ReleaseGroupPrimaryTypeSet;

impl ReleaseGroupPrimaryTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupPrimaryType>> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"SELECT * FROM "musicbrainz"."release_group_primary_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupPrimaryType> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"SELECT * FROM "musicbrainz"."release_group_primary_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupPrimaryType>> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"SELECT * FROM "musicbrainz"."release_group_primary_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupPrimaryType>> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"SELECT * FROM "musicbrainz"."release_group_primary_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_primary_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, release_group_primary_type_id: i32) -> Result<Vec<ReleaseGroupPrimaryType>> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"SELECT * FROM "musicbrainz"."release_group_primary_type" WHERE parent = $1"#)
            .bind(release_group_primary_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_primary_type: ReleaseGroupPrimaryType) -> Result<ReleaseGroupPrimaryType> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"INSERT INTO "release_group_primary_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_group_primary_type.name)
            .bind(release_group_primary_type.parent)
            .bind(release_group_primary_type.child_order)
            .bind(release_group_primary_type.gid)
            .bind(release_group_primary_type.description)
            .bind(release_group_primary_type.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_primary_type: ReleaseGroupPrimaryType) -> Result<ReleaseGroupPrimaryType> {
        query_as::<_, ReleaseGroupPrimaryType>(r#"UPDATE "release_group_primary_type" SET "child_order" = $4, "description" = $5, "name" = $2, "parent" = $3, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_primary_type.gid)
            .bind(release_group_primary_type.id)
            .bind(release_group_primary_type.name)
            .bind(release_group_primary_type.parent)
            .bind(release_group_primary_type.child_order)
            .bind(release_group_primary_type.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_primary_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
