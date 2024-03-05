#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAliasType;

pub struct AreaAliasTypeSet;

impl AreaAliasTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAliasType>> {
        query_as::<_, AreaAliasType>(r#"SELECT * FROM "musicbrainz"."area_alias_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaAliasType> {
        query_as::<_, AreaAliasType>(r#"SELECT * FROM "musicbrainz"."area_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaAliasType>> {
        query_as::<_, AreaAliasType>(r#"SELECT * FROM "musicbrainz"."area_alias_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaAliasType>> {
        query_as::<_, AreaAliasType>(r#"SELECT * FROM "musicbrainz"."area_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_alias_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, area_alias_type_id: i32) -> Result<Vec<AreaAliasType>> {
        query_as::<_, AreaAliasType>(r#"SELECT * FROM "musicbrainz"."area_alias_type" WHERE parent = $1"#)
            .bind(area_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_alias_type: AreaAliasType) -> Result<AreaAliasType> {
        query_as::<_, AreaAliasType>(r#"INSERT INTO "area_alias_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(area_alias_type.id)
            .bind(area_alias_type.name)
            .bind(area_alias_type.description)
            .bind(area_alias_type.gid)
            .bind(area_alias_type.child_order)
            .bind(area_alias_type.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_alias_type: AreaAliasType) -> Result<AreaAliasType> {
        query_as::<_, AreaAliasType>(r#"UPDATE "area_alias_type" SET "description" = $5, "gid" = $6, "parent" = $3, "name" = $2, "child_order" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_alias_type.gid)
            .bind(area_alias_type.name)
            .bind(area_alias_type.id)
            .bind(area_alias_type.parent)
            .bind(area_alias_type.child_order)
            .bind(area_alias_type.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_alias_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
