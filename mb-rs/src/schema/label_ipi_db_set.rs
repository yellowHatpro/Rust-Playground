#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelIpi;

pub struct LabelIpiSet;

impl LabelIpiSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_label_and_ipi<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, ipi: String) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = $1 AND "ipi" = $2"#)
            .bind(label)
            .bind(ipi)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_label_and_ipi_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, ipi_list: Vec<String>) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = ANY($1) AND "ipi" = ANY($2)"#)
            .bind(label_list)
            .bind(ipi_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_label_and_ipi_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, ipi: String) -> Result<Option<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE "label" = $1 AND "ipi" = $2"#)
            .bind(label)
            .bind(ipi)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelIpi>> {
        query_as::<_, LabelIpi>(r#"SELECT * FROM "musicbrainz"."label_ipi" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_ipi: LabelIpi) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"INSERT INTO "label_ipi" ("label", "ipi", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_ipi.ipi)
            .bind(label_ipi.edits_pending)
            .bind(label_ipi.label)
            .bind(label_ipi.created)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_ipi: LabelIpi) -> Result<LabelIpi> {
        query_as::<_, LabelIpi>(r#"UPDATE "label_ipi" SET "created" = $4, "edits_pending" = $3 WHERE "label" = 1 AND "ipi" = 2 RETURNING *;"#)
            .bind(label_ipi.ipi)
            .bind(label_ipi.created)
            .bind(label_ipi.label)
            .bind(label_ipi.edits_pending)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_ipi" WHERE "ipi" = 2 AND "label" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
