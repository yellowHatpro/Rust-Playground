#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentAlias;

pub struct InstrumentAliasSet;

impl InstrumentAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentAlias>> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<InstrumentAlias> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<InstrumentAlias>> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<InstrumentAlias>> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_instrument_id_where_instrument_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<InstrumentAlias>> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, instrument_alias_type_id: i32) -> Result<Vec<InstrumentAlias>> {
        query_as::<_, InstrumentAlias>(r#"SELECT * FROM "musicbrainz"."instrument_alias" WHERE type = $1"#)
            .bind(instrument_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_alias: InstrumentAlias) -> Result<InstrumentAlias> {
        query_as::<_, InstrumentAlias>(r#"INSERT INTO "instrument_alias" ("id", "instrument", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(instrument_alias.id)
            .bind(instrument_alias.edits_pending)
            .bind(instrument_alias.primary_for_locale)
            .bind(instrument_alias.ended)
            .bind(instrument_alias.locale)
            .bind(instrument_alias.end_date_month)
            .bind(instrument_alias.last_updated)
            .bind(instrument_alias.sort_name)
            .bind(instrument_alias.begin_date_day)
            .bind(instrument_alias.Type)
            .bind(instrument_alias.begin_date_month)
            .bind(instrument_alias.begin_date_year)
            .bind(instrument_alias.end_date_year)
            .bind(instrument_alias.end_date_day)
            .bind(instrument_alias.instrument)
            .bind(instrument_alias.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_alias: InstrumentAlias) -> Result<InstrumentAlias> {
        query_as::<_, InstrumentAlias>(r#"UPDATE "instrument_alias" SET "instrument" = $2, "end_date_month" = $13, "primary_for_locale" = $15, "sort_name" = $8, "last_updated" = $6, "name" = $3, "end_date_year" = $12, "end_date_day" = $14, "edits_pending" = $5, "type" = $7, "ended" = $16, "begin_date_month" = $10, "locale" = $4, "begin_date_day" = $11, "begin_date_year" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(instrument_alias.edits_pending)
            .bind(instrument_alias.last_updated)
            .bind(instrument_alias.sort_name)
            .bind(instrument_alias.end_date_month)
            .bind(instrument_alias.ended)
            .bind(instrument_alias.instrument)
            .bind(instrument_alias.Type)
            .bind(instrument_alias.locale)
            .bind(instrument_alias.end_date_year)
            .bind(instrument_alias.begin_date_year)
            .bind(instrument_alias.name)
            .bind(instrument_alias.begin_date_month)
            .bind(instrument_alias.begin_date_day)
            .bind(instrument_alias.end_date_day)
            .bind(instrument_alias.primary_for_locale)
            .bind(instrument_alias.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
