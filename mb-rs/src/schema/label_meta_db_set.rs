#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelMeta;

pub struct LabelMetaSet;

impl LabelMetaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE id = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_meta: LabelMeta) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"INSERT INTO "label_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(label_meta.id)
            .bind(label_meta.rating)
            .bind(label_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_meta: LabelMeta) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"UPDATE "label_meta" SET "rating" = $2, "rating_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_meta.id)
            .bind(label_meta.rating)
            .bind(label_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
