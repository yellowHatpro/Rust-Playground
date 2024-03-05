#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelType;

pub struct LabelTypeSet;

impl LabelTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelType>> {
        query_as::<_, LabelType>(r#"SELECT * FROM "musicbrainz"."label_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelType> {
        query_as::<_, LabelType>(r#"SELECT * FROM "musicbrainz"."label_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelType>> {
        query_as::<_, LabelType>(r#"SELECT * FROM "musicbrainz"."label_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelType>> {
        query_as::<_, LabelType>(r#"SELECT * FROM "musicbrainz"."label_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, label_type_id: i32) -> Result<Vec<LabelType>> {
        query_as::<_, LabelType>(r#"SELECT * FROM "musicbrainz"."label_type" WHERE parent = $1"#)
            .bind(label_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_type: LabelType) -> Result<LabelType> {
        query_as::<_, LabelType>(r#"INSERT INTO "label_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(label_type.gid)
            .bind(label_type.name)
            .bind(label_type.parent)
            .bind(label_type.child_order)
            .bind(label_type.id)
            .bind(label_type.description)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_type: LabelType) -> Result<LabelType> {
        query_as::<_, LabelType>(r#"UPDATE "label_type" SET "parent" = $3, "name" = $2, "description" = $5, "child_order" = $4, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_type.name)
            .bind(label_type.parent)
            .bind(label_type.id)
            .bind(label_type.gid)
            .bind(label_type.description)
            .bind(label_type.child_order)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
