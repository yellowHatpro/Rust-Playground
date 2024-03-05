#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesAlias;

pub struct SeriesAliasSet;

impl SeriesAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesAlias>> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesAlias> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesAlias>> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesAlias>> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_id_where_series_is<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<SeriesAlias>> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, series_alias_type_id: i32) -> Result<Vec<SeriesAlias>> {
        query_as::<_, SeriesAlias>(r#"SELECT * FROM "musicbrainz"."series_alias" WHERE type = $1"#)
            .bind(series_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_alias: SeriesAlias) -> Result<SeriesAlias> {
        query_as::<_, SeriesAlias>(r#"INSERT INTO "series_alias" ("id", "series", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(series_alias.edits_pending)
            .bind(series_alias.sort_name)
            .bind(series_alias.id)
            .bind(series_alias.name)
            .bind(series_alias.begin_date_month)
            .bind(series_alias.begin_date_year)
            .bind(series_alias.end_date_month)
            .bind(series_alias.last_updated)
            .bind(series_alias.primary_for_locale)
            .bind(series_alias.begin_date_day)
            .bind(series_alias.end_date_day)
            .bind(series_alias.Type)
            .bind(series_alias.locale)
            .bind(series_alias.series)
            .bind(series_alias.end_date_year)
            .bind(series_alias.ended)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_alias: SeriesAlias) -> Result<SeriesAlias> {
        query_as::<_, SeriesAlias>(r#"UPDATE "series_alias" SET "type" = $7, "series" = $2, "edits_pending" = $5, "begin_date_month" = $10, "locale" = $4, "primary_for_locale" = $15, "name" = $3, "sort_name" = $8, "last_updated" = $6, "ended" = $16, "begin_date_day" = $11, "end_date_day" = $14, "begin_date_year" = $9, "end_date_year" = $12, "end_date_month" = $13 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_alias.sort_name)
            .bind(series_alias.primary_for_locale)
            .bind(series_alias.Type)
            .bind(series_alias.id)
            .bind(series_alias.end_date_month)
            .bind(series_alias.last_updated)
            .bind(series_alias.begin_date_month)
            .bind(series_alias.end_date_year)
            .bind(series_alias.series)
            .bind(series_alias.begin_date_day)
            .bind(series_alias.begin_date_year)
            .bind(series_alias.edits_pending)
            .bind(series_alias.name)
            .bind(series_alias.ended)
            .bind(series_alias.end_date_day)
            .bind(series_alias.locale)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
