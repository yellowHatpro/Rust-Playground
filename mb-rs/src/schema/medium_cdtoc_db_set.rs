#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumCdtoc;

pub struct MediumCdtocSet;

impl MediumCdtocSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_medium_id_where_medium_is<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_cdtoc_id_where_cdtoc_is<'e, E: PgExecutor<'e>>(executor: E, cdtoc_id: i32) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE cdtoc = $1"#)
            .bind(cdtoc_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_cdtoc: MediumCdtoc) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"INSERT INTO "medium_cdtoc" ("id", "medium", "cdtoc", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(medium_cdtoc.medium)
            .bind(medium_cdtoc.id)
            .bind(medium_cdtoc.edits_pending)
            .bind(medium_cdtoc.last_updated)
            .bind(medium_cdtoc.cdtoc)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_cdtoc: MediumCdtoc) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"UPDATE "medium_cdtoc" SET "last_updated" = $5, "medium" = $2, "edits_pending" = $4, "cdtoc" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium_cdtoc.medium)
            .bind(medium_cdtoc.last_updated)
            .bind(medium_cdtoc.id)
            .bind(medium_cdtoc.edits_pending)
            .bind(medium_cdtoc.cdtoc)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_cdtoc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
