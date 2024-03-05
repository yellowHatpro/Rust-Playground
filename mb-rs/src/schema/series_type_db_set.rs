#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesType;

pub struct SeriesTypeSet;

impl SeriesTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesType>> {
        query_as::<_, SeriesType>(r#"SELECT * FROM "musicbrainz"."series_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesType> {
        query_as::<_, SeriesType>(r#"SELECT * FROM "musicbrainz"."series_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesType>> {
        query_as::<_, SeriesType>(r#"SELECT * FROM "musicbrainz"."series_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesType>> {
        query_as::<_, SeriesType>(r#"SELECT * FROM "musicbrainz"."series_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, series_type_id: i32) -> Result<Vec<SeriesType>> {
        query_as::<_, SeriesType>(r#"SELECT * FROM "musicbrainz"."series_type" WHERE parent = $1"#)
            .bind(series_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_type: SeriesType) -> Result<SeriesType> {
        query_as::<_, SeriesType>(r#"INSERT INTO "series_type" ("id", "name", "entity_type", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(series_type.description)
            .bind(series_type.id)
            .bind(series_type.parent)
            .bind(series_type.child_order)
            .bind(series_type.gid)
            .bind(series_type.name)
            .bind(series_type.entity_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_type: SeriesType) -> Result<SeriesType> {
        query_as::<_, SeriesType>(r#"UPDATE "series_type" SET "gid" = $7, "child_order" = $5, "parent" = $4, "description" = $6, "name" = $2, "entity_type" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_type.child_order)
            .bind(series_type.gid)
            .bind(series_type.description)
            .bind(series_type.id)
            .bind(series_type.entity_type)
            .bind(series_type.parent)
            .bind(series_type.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
