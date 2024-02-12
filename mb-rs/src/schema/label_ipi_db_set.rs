#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelIpi;

pub struct LabelIpiSet;

impl LabelIpiSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_ipi<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, ipi: char) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = $1 AND "ipi" = $2"#)
            .bind(label)
            .bind(ipi)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_label_and_ipi_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, ipi_list: Vec<char>) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = ANY($1) AND "ipi" = ANY($2)"#)
            .bind(label_list)
            .bind(ipi_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_ipi_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, ipi: char) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = $1 AND "ipi" = $2"#)
            .bind(label)
            .bind(ipi)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_ipi: LabelIpi) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"INSERT INTO "label_ipi" ("label", "ipi", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_ipi.label)
            .bind(label_ipi.ipi)
            .bind(label_ipi.edits_pending)
            .bind(label_ipi.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_ipi: LabelIpi) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"UPDATE "label_ipi" SET "edits_pending" = $3, "created" = $4 WHERE "label" = 1 AND "ipi" = 2 RETURNING *;"#)
            .bind(label_ipi.label)
            .bind(label_ipi.ipi)
            .bind(label_ipi.edits_pending)
            .bind(label_ipi.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_ipi" WHERE "label" = 1 AND "ipi" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
