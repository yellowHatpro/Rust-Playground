#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelIsni;

pub struct LabelIsniSet;

impl LabelIsniSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_label_and_isni<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, isni: String) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = $1 AND "isni" = $2"#)
            .bind(label)
            .bind(isni)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_label_and_isni_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, isni_list: Vec<String>) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = ANY($1) AND "isni" = ANY($2)"#)
            .bind(label_list)
            .bind(isni_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_label_and_isni_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, isni: String) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = $1 AND "isni" = $2"#)
            .bind(label)
            .bind(isni)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_isni: LabelIsni) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"INSERT INTO "label_isni" ("label", "isni", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_isni.label)
            .bind(label_isni.created)
            .bind(label_isni.edits_pending)
            .bind(label_isni.isni)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_isni: LabelIsni) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"UPDATE "label_isni" SET "created" = $4, "edits_pending" = $3 WHERE "label" = 1 AND "isni" = 2 RETURNING *;"#)
            .bind(label_isni.label)
            .bind(label_isni.edits_pending)
            .bind(label_isni.created)
            .bind(label_isni.isni)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_isni" WHERE "label" = 1 AND "isni" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
