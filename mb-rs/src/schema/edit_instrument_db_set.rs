#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditInstrument;

pub struct EditInstrumentSet;

impl EditInstrumentSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditInstrument>> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_instrument<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, instrument: i32) -> Result<EditInstrument> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument" WHERE "edit" = $1 AND "instrument" = $2"#)
            .bind(edit)
            .bind(instrument)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_instrument_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, instrument_list: Vec<i32>) -> Result<Vec<EditInstrument>> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument" WHERE "edit" = ANY($1) AND "instrument" = ANY($2)"#)
            .bind(edit_list)
            .bind(instrument_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_instrument_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, instrument: i32) -> Result<Option<EditInstrument>> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument" WHERE "edit" = $1 AND "instrument" = $2"#)
            .bind(edit)
            .bind(instrument)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditInstrument>> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_id_where_instrument_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<EditInstrument>> {
        query_as::<_, EditInstrument>(r#"SELECT * FROM "musicbrainz"."edit_instrument" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_instrument: EditInstrument) -> Result<EditInstrument> {
        query_as::<_, EditInstrument>(r#"INSERT INTO "edit_instrument" ("edit", "instrument") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_instrument.instrument)
            .bind(edit_instrument.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_instrument: EditInstrument) -> Result<EditInstrument> {
        query_as::<_, EditInstrument>(r#"UPDATE "edit_instrument" SET  WHERE "edit" = 1 AND "instrument" = 2 RETURNING *;"#)
            .bind(edit_instrument.edit)
            .bind(edit_instrument.instrument)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_instrument" WHERE "instrument" = 2 AND "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
