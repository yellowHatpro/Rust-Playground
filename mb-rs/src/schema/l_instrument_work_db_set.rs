#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LInstrumentWork;

pub struct LInstrumentWorkSet;

impl LInstrumentWorkSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LInstrumentWork> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE entity0 = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<LInstrumentWork>> {
        query_as::<_, LInstrumentWork>(r#"SELECT * FROM "musicbrainz"."l_instrument_work" WHERE entity1 = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_instrument_work: LInstrumentWork) -> Result<LInstrumentWork> {
        query_as::<_, LInstrumentWork>(r#"INSERT INTO "l_instrument_work" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_instrument_work.entity1_credit)
            .bind(l_instrument_work.entity0_credit)
            .bind(l_instrument_work.id)
            .bind(l_instrument_work.entity0)
            .bind(l_instrument_work.entity1)
            .bind(l_instrument_work.link)
            .bind(l_instrument_work.edits_pending)
            .bind(l_instrument_work.last_updated)
            .bind(l_instrument_work.link_order)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_instrument_work: LInstrumentWork) -> Result<LInstrumentWork> {
        query_as::<_, LInstrumentWork>(r#"UPDATE "l_instrument_work" SET "edits_pending" = $5, "last_updated" = $6, "link_order" = $7, "link" = $2, "entity0_credit" = $8, "entity1" = $4, "entity0" = $3, "entity1_credit" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_instrument_work.edits_pending)
            .bind(l_instrument_work.link_order)
            .bind(l_instrument_work.id)
            .bind(l_instrument_work.entity0_credit)
            .bind(l_instrument_work.entity1)
            .bind(l_instrument_work.link)
            .bind(l_instrument_work.entity0)
            .bind(l_instrument_work.entity1_credit)
            .bind(l_instrument_work.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_instrument_work" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
