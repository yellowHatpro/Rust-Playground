#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Instrument;

pub struct InstrumentSet;

impl InstrumentSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_instrument_type_id<'e, E: PgExecutor<'e>>(executor: E, instrument_type_id: i32) -> Result<Vec<Instrument>> {
        query_as::<_, Instrument>(r#"SELECT * FROM "musicbrainz"."instrument" WHERE type = $1"#)
            .bind(instrument_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: Instrument) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"INSERT INTO "instrument" ("id", "gid", "name", "type", "edits_pending", "last_updated", "comment", "description") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(instrument.id)
            .bind(instrument.gid)
            .bind(instrument.name)
            .bind(instrument.Type)
            .bind(instrument.edits_pending)
            .bind(instrument.last_updated)
            .bind(instrument.comment)
            .bind(instrument.description)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: Instrument) -> Result<Instrument> {
        query_as::<_, Instrument>(r#"UPDATE "instrument" SET "gid" = $2, "name" = $3, "type" = $4, "edits_pending" = $5, "last_updated" = $6, "comment" = $7, "description" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(instrument.id)
            .bind(instrument.gid)
            .bind(instrument.name)
            .bind(instrument.Type)
            .bind(instrument.edits_pending)
            .bind(instrument.last_updated)
            .bind(instrument.comment)
            .bind(instrument.description)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
