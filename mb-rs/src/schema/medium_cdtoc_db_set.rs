#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumCdtoc;

pub struct MediumCdtocSet;

impl MediumCdtocSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_medium_id<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_cdtoc_id<'e, E: PgExecutor<'e>>(executor: E, cdtoc_id: i32) -> Result<Vec<MediumCdtoc>> {
        query_as::<_, MediumCdtoc>(r#"SELECT * FROM "musicbrainz"."medium_cdtoc" WHERE cdtoc = $1"#)
            .bind(cdtoc_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_cdtoc: MediumCdtoc) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"INSERT INTO "medium_cdtoc" ("id", "medium", "cdtoc", "edits_pending", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(medium_cdtoc.id)
            .bind(medium_cdtoc.medium)
            .bind(medium_cdtoc.cdtoc)
            .bind(medium_cdtoc.edits_pending)
            .bind(medium_cdtoc.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_cdtoc: MediumCdtoc) -> Result<MediumCdtoc> {
        query_as::<_, MediumCdtoc>(r#"UPDATE "medium_cdtoc" SET "medium" = $2, "cdtoc" = $3, "edits_pending" = $4, "last_updated" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium_cdtoc.id)
            .bind(medium_cdtoc.medium)
            .bind(medium_cdtoc.cdtoc)
            .bind(medium_cdtoc.edits_pending)
            .bind(medium_cdtoc.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_cdtoc" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
