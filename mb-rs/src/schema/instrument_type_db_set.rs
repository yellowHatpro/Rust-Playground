#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentType;

pub struct InstrumentTypeSet;

impl InstrumentTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentType>> {
        query_as::<_, InstrumentType>(r#"SELECT * FROM "musicbrainz"."instrument_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<InstrumentType> {
        query_as::<_, InstrumentType>(r#"SELECT * FROM "musicbrainz"."instrument_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<InstrumentType>> {
        query_as::<_, InstrumentType>(r#"SELECT * FROM "musicbrainz"."instrument_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<InstrumentType>> {
        query_as::<_, InstrumentType>(r#"SELECT * FROM "musicbrainz"."instrument_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_instrument_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, instrument_type_id: i32) -> Result<Vec<InstrumentType>> {
        query_as::<_, InstrumentType>(r#"SELECT * FROM "musicbrainz"."instrument_type" WHERE parent = $1"#)
            .bind(instrument_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_type: InstrumentType) -> Result<InstrumentType> {
        query_as::<_, InstrumentType>(r#"INSERT INTO "instrument_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(instrument_type.parent)
            .bind(instrument_type.gid)
            .bind(instrument_type.name)
            .bind(instrument_type.description)
            .bind(instrument_type.id)
            .bind(instrument_type.child_order)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_type: InstrumentType) -> Result<InstrumentType> {
        query_as::<_, InstrumentType>(r#"UPDATE "instrument_type" SET "parent" = $3, "child_order" = $4, "gid" = $6, "name" = $2, "description" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(instrument_type.id)
            .bind(instrument_type.parent)
            .bind(instrument_type.gid)
            .bind(instrument_type.child_order)
            .bind(instrument_type.name)
            .bind(instrument_type.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
