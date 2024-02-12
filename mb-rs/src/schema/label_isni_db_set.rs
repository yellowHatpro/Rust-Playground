#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelIsni;

pub struct LabelIsniSet;

impl LabelIsniSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_isni<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, isni: char) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = $1 AND "isni" = $2"#)
            .bind(label)
            .bind(isni)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_label_and_isni_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, isni_list: Vec<char>) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = ANY($1) AND "isni" = ANY($2)"#)
            .bind(label_list)
            .bind(isni_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_isni_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, isni: char) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "label" = $1 AND "isni" = $2"#)
            .bind(label)
            .bind(isni)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelIsni>> {
        query_as::<_, LabelIsni>(r#"SELECT * FROM "musicbrainz"."label_isni" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_isni: LabelIsni) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"INSERT INTO "label_isni" ("label", "isni", "edits_pending", "created") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_isni.label)
            .bind(label_isni.isni)
            .bind(label_isni.edits_pending)
            .bind(label_isni.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_isni: LabelIsni) -> Result<LabelIsni> {
        query_as::<_, LabelIsni>(r#"UPDATE "label_isni" SET "edits_pending" = $3, "created" = $4 WHERE "label" = 1 AND "isni" = 2 RETURNING *;"#)
            .bind(label_isni.label)
            .bind(label_isni.isni)
            .bind(label_isni.edits_pending)
            .bind(label_isni.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_isni" WHERE "label" = 1 AND "isni" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
