#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesAliasType;

pub struct SeriesAliasTypeSet;

impl SeriesAliasTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesAliasType>> {
        query_as::<_, SeriesAliasType>(r#"SELECT * FROM "musicbrainz"."series_alias_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesAliasType> {
        query_as::<_, SeriesAliasType>(r#"SELECT * FROM "musicbrainz"."series_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesAliasType>> {
        query_as::<_, SeriesAliasType>(r#"SELECT * FROM "musicbrainz"."series_alias_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesAliasType>> {
        query_as::<_, SeriesAliasType>(r#"SELECT * FROM "musicbrainz"."series_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_alias_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, series_alias_type_id: i32) -> Result<Vec<SeriesAliasType>> {
        query_as::<_, SeriesAliasType>(r#"SELECT * FROM "musicbrainz"."series_alias_type" WHERE parent = $1"#)
            .bind(series_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_alias_type: SeriesAliasType) -> Result<SeriesAliasType> {
        query_as::<_, SeriesAliasType>(r#"INSERT INTO "series_alias_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(series_alias_type.child_order)
            .bind(series_alias_type.description)
            .bind(series_alias_type.name)
            .bind(series_alias_type.id)
            .bind(series_alias_type.gid)
            .bind(series_alias_type.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_alias_type: SeriesAliasType) -> Result<SeriesAliasType> {
        query_as::<_, SeriesAliasType>(r#"UPDATE "series_alias_type" SET "name" = $2, "child_order" = $4, "parent" = $3, "gid" = $6, "description" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_alias_type.parent)
            .bind(series_alias_type.child_order)
            .bind(series_alias_type.name)
            .bind(series_alias_type.id)
            .bind(series_alias_type.description)
            .bind(series_alias_type.gid)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_alias_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
