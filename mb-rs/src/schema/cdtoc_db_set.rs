#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Cdtoc;

pub struct CdtocSet;

impl CdtocSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Cdtoc>> {
        query_as::<_, Cdtoc>(r#"SELECT * FROM "musicbrainz"."cdtoc""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Cdtoc> {
        query_as::<_, Cdtoc>(r#"SELECT * FROM "musicbrainz"."cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Cdtoc>> {
        query_as::<_, Cdtoc>(r#"SELECT * FROM "musicbrainz"."cdtoc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Cdtoc>> {
        query_as::<_, Cdtoc>(r#"SELECT * FROM "musicbrainz"."cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, cdtoc: Cdtoc) -> Result<Cdtoc> {
        query_as::<_, Cdtoc>(r#"INSERT INTO "cdtoc" ("id", "discid", "freedb_id", "track_count", "leadout_offset", "track_offset", "created") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(cdtoc.created)
            .bind(cdtoc.discid)
            .bind(cdtoc.id)
            .bind(cdtoc.freedb_id)
            .bind(cdtoc.leadout_offset)
            .bind(cdtoc.track_offset)
            .bind(cdtoc.track_count)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, cdtoc: Cdtoc) -> Result<Cdtoc> {
        query_as::<_, Cdtoc>(r#"UPDATE "cdtoc" SET "discid" = $2, "leadout_offset" = $5, "track_offset" = $6, "freedb_id" = $3, "track_count" = $4, "created" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(cdtoc.track_offset)
            .bind(cdtoc.id)
            .bind(cdtoc.track_count)
            .bind(cdtoc.created)
            .bind(cdtoc.discid)
            .bind(cdtoc.leadout_offset)
            .bind(cdtoc.freedb_id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."cdtoc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
