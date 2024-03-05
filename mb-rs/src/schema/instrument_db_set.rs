#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Instrument;

pub struct InstrumentSet;

impl InstrumentSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_instrument_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, instrument_type_id: i32) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE type = $1"#)
            .bind(instrument_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: Instrument) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"INSERT INTO "instrument" ("id", "gid", "name", "type", "edits_pending", "last_updated", "comment", "description") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(instrument.Type)
            .bind(instrument.name)
            .bind(instrument.edits_pending)
            .bind(instrument.comment)
            .bind(instrument.last_updated)
            .bind(instrument.gid)
            .bind(instrument.description)
            .bind(instrument.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: Instrument) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"UPDATE "instrument" SET "description" = $8, "gid" = $2, "name" = $3, "edits_pending" = $5, "type" = $4, "last_updated" = $6, "comment" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(instrument.Type)
            .bind(instrument.description)
            .bind(instrument.gid)
            .bind(instrument.id)
            .bind(instrument.edits_pending)
            .bind(instrument.name)
            .bind(instrument.last_updated)
            .bind(instrument.comment)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
