#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumFormat;

pub struct MediumFormatSet;

impl MediumFormatSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumFormat>> {
        query_as::<_, MediumFormat>(r#"SELECT * FROM "musicbrainz"."medium_format""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<MediumFormat> {
        query_as::<_, MediumFormat>(r#"SELECT * FROM "musicbrainz"."medium_format" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<MediumFormat>> {
        query_as::<_, MediumFormat>(r#"SELECT * FROM "musicbrainz"."medium_format" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<MediumFormat>> {
        query_as::<_, MediumFormat>(r#"SELECT * FROM "musicbrainz"."medium_format" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_medium_format_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, medium_format_id: i32) -> Result<Vec<MediumFormat>> {
        query_as::<_, MediumFormat>(r#"SELECT * FROM "musicbrainz"."medium_format" WHERE parent = $1"#)
            .bind(medium_format_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: MediumFormat) -> Result<MediumFormat> {
        query_as::<_, MediumFormat>(r#"INSERT INTO "medium_format" ("id", "name", "parent", "child_order", "year", "has_discids", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(medium_format.parent)
            .bind(medium_format.year)
            .bind(medium_format.name)
            .bind(medium_format.id)
            .bind(medium_format.gid)
            .bind(medium_format.description)
            .bind(medium_format.child_order)
            .bind(medium_format.has_discids)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_format: MediumFormat) -> Result<MediumFormat> {
        query_as::<_, MediumFormat>(r#"UPDATE "medium_format" SET "name" = $2, "has_discids" = $6, "parent" = $3, "description" = $7, "gid" = $8, "child_order" = $4, "year" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium_format.name)
            .bind(medium_format.description)
            .bind(medium_format.id)
            .bind(medium_format.parent)
            .bind(medium_format.gid)
            .bind(medium_format.child_order)
            .bind(medium_format.has_discids)
            .bind(medium_format.year)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_format" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
