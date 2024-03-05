#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAlias;

pub struct AreaAliasSet;

impl AreaAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAlias>> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaAlias> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaAlias>> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaAlias>> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaAlias>> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, area_alias_type_id: i32) -> Result<Vec<AreaAlias>> {
        query_as::<_, AreaAlias>(r#"SELECT * FROM "musicbrainz"."area_alias" WHERE type = $1"#)
            .bind(area_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_alias: AreaAlias) -> Result<AreaAlias> {
        query_as::<_, AreaAlias>(r#"INSERT INTO "area_alias" ("id", "area", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(area_alias.name)
            .bind(area_alias.Type)
            .bind(area_alias.end_date_year)
            .bind(area_alias.primary_for_locale)
            .bind(area_alias.area)
            .bind(area_alias.id)
            .bind(area_alias.last_updated)
            .bind(area_alias.end_date_month)
            .bind(area_alias.begin_date_year)
            .bind(area_alias.sort_name)
            .bind(area_alias.locale)
            .bind(area_alias.begin_date_day)
            .bind(area_alias.end_date_day)
            .bind(area_alias.edits_pending)
            .bind(area_alias.begin_date_month)
            .bind(area_alias.ended)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_alias: AreaAlias) -> Result<AreaAlias> {
        query_as::<_, AreaAlias>(r#"UPDATE "area_alias" SET "begin_date_year" = $9, "name" = $3, "end_date_month" = $13, "sort_name" = $8, "begin_date_month" = $10, "begin_date_day" = $11, "end_date_day" = $14, "primary_for_locale" = $15, "locale" = $4, "last_updated" = $6, "end_date_year" = $12, "edits_pending" = $5, "type" = $7, "ended" = $16, "area" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_alias.edits_pending)
            .bind(area_alias.Type)
            .bind(area_alias.end_date_month)
            .bind(area_alias.ended)
            .bind(area_alias.area)
            .bind(area_alias.primary_for_locale)
            .bind(area_alias.begin_date_year)
            .bind(area_alias.locale)
            .bind(area_alias.end_date_day)
            .bind(area_alias.last_updated)
            .bind(area_alias.sort_name)
            .bind(area_alias.begin_date_month)
            .bind(area_alias.end_date_year)
            .bind(area_alias.begin_date_day)
            .bind(area_alias.name)
            .bind(area_alias.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
