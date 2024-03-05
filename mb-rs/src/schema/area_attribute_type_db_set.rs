#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAttributeType;

pub struct AreaAttributeTypeSet;

impl AreaAttributeTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAttributeType>> {
        query_as::<_, AreaAttributeType>(r#"SELECT * FROM "musicbrainz"."area_attribute_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaAttributeType> {
        query_as::<_, AreaAttributeType>(r#"SELECT * FROM "musicbrainz"."area_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaAttributeType>> {
        query_as::<_, AreaAttributeType>(r#"SELECT * FROM "musicbrainz"."area_attribute_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaAttributeType>> {
        query_as::<_, AreaAttributeType>(r#"SELECT * FROM "musicbrainz"."area_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_attribute_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, area_attribute_type_id: i32) -> Result<Vec<AreaAttributeType>> {
        query_as::<_, AreaAttributeType>(r#"SELECT * FROM "musicbrainz"."area_attribute_type" WHERE parent = $1"#)
            .bind(area_attribute_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute_type: AreaAttributeType) -> Result<AreaAttributeType> {
        query_as::<_, AreaAttributeType>(r#"INSERT INTO "area_attribute_type" ("id", "name", "comment", "free_text", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(area_attribute_type.free_text)
            .bind(area_attribute_type.child_order)
            .bind(area_attribute_type.description)
            .bind(area_attribute_type.name)
            .bind(area_attribute_type.comment)
            .bind(area_attribute_type.gid)
            .bind(area_attribute_type.id)
            .bind(area_attribute_type.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute_type: AreaAttributeType) -> Result<AreaAttributeType> {
        query_as::<_, AreaAttributeType>(r#"UPDATE "area_attribute_type" SET "parent" = $5, "description" = $7, "comment" = $3, "name" = $2, "child_order" = $6, "free_text" = $4, "gid" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_attribute_type.parent)
            .bind(area_attribute_type.name)
            .bind(area_attribute_type.child_order)
            .bind(area_attribute_type.description)
            .bind(area_attribute_type.free_text)
            .bind(area_attribute_type.comment)
            .bind(area_attribute_type.gid)
            .bind(area_attribute_type.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_attribute_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
