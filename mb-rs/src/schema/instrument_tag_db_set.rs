#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentTag;

pub struct InstrumentTagSet;

impl InstrumentTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_instrument_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, tag: i32) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = $1 AND "tag" = $2"#)
            .bind(instrument)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_instrument_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(instrument_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_instrument_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, tag: i32) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "instrument" = $1 AND "tag" = $2"#)
            .bind(instrument)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_instrument_id<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<InstrumentTag>> {
        query_as::<_, InstrumentTag>(r#"SELECT * FROM "musicbrainz"."instrument_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_tag: InstrumentTag) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"INSERT INTO "instrument_tag" ("instrument", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(instrument_tag.instrument)
            .bind(instrument_tag.tag)
            .bind(instrument_tag.count)
            .bind(instrument_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_tag: InstrumentTag) -> Result<InstrumentTag> {
        query_as::<_, InstrumentTag>(r#"UPDATE "instrument_tag" SET "count" = $3, "last_updated" = $4 WHERE "instrument" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(instrument_tag.instrument)
            .bind(instrument_tag.tag)
            .bind(instrument_tag.count)
            .bind(instrument_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_tag" WHERE "instrument" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
