#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LGenreInstrument;

pub struct LGenreInstrumentSet;

impl LGenreInstrumentSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LGenreInstrument> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE entity0 = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<LGenreInstrument>> {
        query_as::<_, LGenreInstrument>(r#"SELECT * FROM "musicbrainz"."l_genre_instrument" WHERE entity1 = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_instrument: LGenreInstrument) -> Result<LGenreInstrument> {
        query_as::<_, LGenreInstrument>(r#"INSERT INTO "l_genre_instrument" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_genre_instrument.link_order)
            .bind(l_genre_instrument.entity1_credit)
            .bind(l_genre_instrument.id)
            .bind(l_genre_instrument.entity1)
            .bind(l_genre_instrument.entity0)
            .bind(l_genre_instrument.link)
            .bind(l_genre_instrument.edits_pending)
            .bind(l_genre_instrument.last_updated)
            .bind(l_genre_instrument.entity0_credit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_instrument: LGenreInstrument) -> Result<LGenreInstrument> {
        query_as::<_, LGenreInstrument>(r#"UPDATE "l_genre_instrument" SET "edits_pending" = $5, "link" = $2, "entity1" = $4, "entity1_credit" = $9, "entity0_credit" = $8, "last_updated" = $6, "link_order" = $7, "entity0" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_genre_instrument.link_order)
            .bind(l_genre_instrument.entity1)
            .bind(l_genre_instrument.edits_pending)
            .bind(l_genre_instrument.last_updated)
            .bind(l_genre_instrument.id)
            .bind(l_genre_instrument.entity0)
            .bind(l_genre_instrument.link)
            .bind(l_genre_instrument.entity1_credit)
            .bind(l_genre_instrument.entity0_credit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_genre_instrument" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
