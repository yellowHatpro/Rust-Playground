#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentAttribute;

pub struct InstrumentAttributeSet;

impl InstrumentAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<InstrumentAttribute> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_instrument_id_where_instrument_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_attribute_type_id_where_instrument_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, instrument_attribute_type_id: i32) -> Result<Vec<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE instrument_attribute_type = $1"#)
            .bind(instrument_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_attribute_type_allowed_value_id_where_instrument_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, instrument_attribute_type_allowed_value_id: i32) -> Result<Vec<InstrumentAttribute>> {
        query_as::<_, InstrumentAttribute>(r#"SELECT * FROM "musicbrainz"."instrument_attribute" WHERE instrument_attribute_type_allowed_value = $1"#)
            .bind(instrument_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_attribute: InstrumentAttribute) -> Result<InstrumentAttribute> {
        query_as::<_, InstrumentAttribute>(r#"INSERT INTO "instrument_attribute" ("id", "instrument", "instrument_attribute_type", "instrument_attribute_type_allowed_value", "instrument_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(instrument_attribute.instrument_attribute_text)
            .bind(instrument_attribute.instrument)
            .bind(instrument_attribute.id)
            .bind(instrument_attribute.instrument_attribute_type)
            .bind(instrument_attribute.instrument_attribute_type_allowed_value)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_attribute: InstrumentAttribute) -> Result<InstrumentAttribute> {
        query_as::<_, InstrumentAttribute>(r#"UPDATE "instrument_attribute" SET "instrument_attribute_type" = $3, "instrument_attribute_type_allowed_value" = $4, "instrument_attribute_text" = $5, "instrument" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(instrument_attribute.instrument_attribute_type_allowed_value)
            .bind(instrument_attribute.id)
            .bind(instrument_attribute.instrument)
            .bind(instrument_attribute.instrument_attribute_type)
            .bind(instrument_attribute.instrument_attribute_text)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
