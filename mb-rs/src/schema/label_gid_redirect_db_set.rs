#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelGidRedirect;

pub struct LabelGidRedirectSet;

impl LabelGidRedirectSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, gid_list: Vec<uuid::Uuid>) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "gid" = ANY($1)"#)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<Option<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelGidRedirect>> {
        query_as::<_, LabelGidRedirect>(r#"SELECT * FROM "musicbrainz"."label_gid_redirect" WHERE new_id = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_gid_redirect: LabelGidRedirect) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"INSERT INTO "label_gid_redirect" ("gid", "new_id", "created") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(label_gid_redirect.gid)
            .bind(label_gid_redirect.new_id)
            .bind(label_gid_redirect.created)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_gid_redirect: LabelGidRedirect) -> Result<LabelGidRedirect> {
        query_as::<_, LabelGidRedirect>(r#"UPDATE "label_gid_redirect" SET "new_id" = $2, "created" = $3 WHERE "gid" = 1 RETURNING *;"#)
            .bind(label_gid_redirect.gid)
            .bind(label_gid_redirect.new_id)
            .bind(label_gid_redirect.created)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_gid_redirect" WHERE "gid" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
