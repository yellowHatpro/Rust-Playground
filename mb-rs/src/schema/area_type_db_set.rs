#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaType;

pub struct AreaTypeSet;

impl AreaTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaType>> {
        query_as::<_, AreaType>(r#"SELECT * FROM "musicbrainz"."area_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaType> {
        query_as::<_, AreaType>(r#"SELECT * FROM "musicbrainz"."area_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaType>> {
        query_as::<_, AreaType>(r#"SELECT * FROM "musicbrainz"."area_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaType>> {
        query_as::<_, AreaType>(r#"SELECT * FROM "musicbrainz"."area_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, area_type_id: i32) -> Result<Vec<AreaType>> {
        query_as::<_, AreaType>(r#"SELECT * FROM "musicbrainz"."area_type" WHERE parent = $1"#)
            .bind(area_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_type: AreaType) -> Result<AreaType> {
        query_as::<_, AreaType>(r#"INSERT INTO "area_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(area_type.parent)
            .bind(area_type.gid)
            .bind(area_type.id)
            .bind(area_type.name)
            .bind(area_type.description)
            .bind(area_type.child_order)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_type: AreaType) -> Result<AreaType> {
        query_as::<_, AreaType>(r#"UPDATE "area_type" SET "child_order" = $4, "description" = $5, "parent" = $3, "gid" = $6, "name" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_type.parent)
            .bind(area_type.name)
            .bind(area_type.id)
            .bind(area_type.description)
            .bind(area_type.gid)
            .bind(area_type.child_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
