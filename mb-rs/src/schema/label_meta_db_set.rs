#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelMeta;

pub struct LabelMetaSet;

impl LabelMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelMeta>> {
        query_as::<_, LabelMeta>(r#"SELECT * FROM "musicbrainz"."label_meta" WHERE id = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_meta: LabelMeta) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"INSERT INTO "label_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(label_meta.id)
            .bind(label_meta.rating_count)
            .bind(label_meta.rating)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_meta: LabelMeta) -> Result<LabelMeta> {
        query_as::<_, LabelMeta>(r#"UPDATE "label_meta" SET "rating" = $2, "rating_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_meta.rating)
            .bind(label_meta.rating_count)
            .bind(label_meta.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
