#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CdtocRaw;

pub struct CdtocRawSet;

impl CdtocRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CdtocRaw>> {
        query_as::<_, CdtocRaw>(r#"SELECT * FROM "musicbrainz"."cdtoc_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<CdtocRaw> {
        query_as::<_, CdtocRaw>(r#"SELECT * FROM "musicbrainz"."cdtoc_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<CdtocRaw>> {
        query_as::<_, CdtocRaw>(r#"SELECT * FROM "musicbrainz"."cdtoc_raw" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<CdtocRaw>> {
        query_as::<_, CdtocRaw>(r#"SELECT * FROM "musicbrainz"."cdtoc_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_raw_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_raw_id: i32) -> Result<Vec<CdtocRaw>> {
        query_as::<_, CdtocRaw>(r#"SELECT * FROM "musicbrainz"."cdtoc_raw" WHERE release = $1"#)
            .bind(release_raw_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, cdtoc_raw: CdtocRaw) -> Result<CdtocRaw> {
        query_as::<_, CdtocRaw>(r#"INSERT INTO "cdtoc_raw" ("id", "release", "discid", "track_count", "leadout_offset", "track_offset") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(cdtoc_raw.release)
            .bind(cdtoc_raw.leadout_offset)
            .bind(cdtoc_raw.track_offset)
            .bind(cdtoc_raw.discid)
            .bind(cdtoc_raw.id)
            .bind(cdtoc_raw.track_count)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, cdtoc_raw: CdtocRaw) -> Result<CdtocRaw> {
        query_as::<_, CdtocRaw>(r#"UPDATE "cdtoc_raw" SET "track_offset" = $6, "release" = $2, "discid" = $3, "leadout_offset" = $5, "track_count" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(cdtoc_raw.id)
            .bind(cdtoc_raw.release)
            .bind(cdtoc_raw.discid)
            .bind(cdtoc_raw.track_count)
            .bind(cdtoc_raw.leadout_offset)
            .bind(cdtoc_raw.track_offset)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."cdtoc_raw" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
