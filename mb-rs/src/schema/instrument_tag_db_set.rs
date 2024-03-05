#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentTag;

pub struct InstrumentTagSet;

impl InstrumentTagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_instrument_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, tag: i32) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = $1 AND "tag" = $2"#)
            .bind(instrument)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_instrument_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(instrument_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_instrument_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, tag: i32) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = $1 AND "tag" = $2"#)
            .bind(instrument)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_instrument_id_where_instrument_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_tag: InstrumentTag) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"INSERT INTO "instrument_tag" ("instrument", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(instrument_tag.instrument)
            .bind(instrument_tag.count)
            .bind(instrument_tag.tag)
            .bind(instrument_tag.last_updated)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_tag: InstrumentTag) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"UPDATE "instrument_tag" SET "last_updated" = $4, "count" = $3 WHERE "instrument" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(instrument_tag.count)
            .bind(instrument_tag.tag)
            .bind(instrument_tag.last_updated)
            .bind(instrument_tag.instrument)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_tag" WHERE "tag" = 2 AND "instrument" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
