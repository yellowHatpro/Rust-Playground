#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesOrderingType;

pub struct SeriesOrderingTypeSet;

impl SeriesOrderingTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesOrderingType>> {
        query_as::<_, SeriesOrderingType>(r#"SELECT * FROM "musicbrainz"."series_ordering_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesOrderingType> {
        query_as::<_, SeriesOrderingType>(r#"SELECT * FROM "musicbrainz"."series_ordering_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesOrderingType>> {
        query_as::<_, SeriesOrderingType>(r#"SELECT * FROM "musicbrainz"."series_ordering_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesOrderingType>> {
        query_as::<_, SeriesOrderingType>(r#"SELECT * FROM "musicbrainz"."series_ordering_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_ordering_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, series_ordering_type_id: i32) -> Result<Vec<SeriesOrderingType>> {
        query_as::<_, SeriesOrderingType>(r#"SELECT * FROM "musicbrainz"."series_ordering_type" WHERE parent = $1"#)
            .bind(series_ordering_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_ordering_type: SeriesOrderingType) -> Result<SeriesOrderingType> {
        query_as::<_, SeriesOrderingType>(r#"INSERT INTO "series_ordering_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(series_ordering_type.description)
            .bind(series_ordering_type.parent)
            .bind(series_ordering_type.name)
            .bind(series_ordering_type.id)
            .bind(series_ordering_type.child_order)
            .bind(series_ordering_type.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_ordering_type: SeriesOrderingType) -> Result<SeriesOrderingType> {
        query_as::<_, SeriesOrderingType>(r#"UPDATE "series_ordering_type" SET "description" = $5, "parent" = $3, "gid" = $6, "name" = $2, "child_order" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_ordering_type.child_order)
            .bind(series_ordering_type.id)
            .bind(series_ordering_type.description)
            .bind(series_ordering_type.gid)
            .bind(series_ordering_type.parent)
            .bind(series_ordering_type.name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_ordering_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
