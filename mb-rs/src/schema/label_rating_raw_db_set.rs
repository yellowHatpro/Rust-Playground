#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelRatingRaw;

pub struct LabelRatingRawSet;

impl LabelRatingRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelRatingRaw>> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_label_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, editor: i32) -> Result<LabelRatingRaw> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw" WHERE "label" = $1 AND "editor" = $2"#)
            .bind(label)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_label_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<LabelRatingRaw>> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw" WHERE "label" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(label_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_label_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, editor: i32) -> Result<Option<LabelRatingRaw>> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw" WHERE "label" = $1 AND "editor" = $2"#)
            .bind(label)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelRatingRaw>> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<LabelRatingRaw>> {
        query_as::<_, LabelRatingRaw>(r#"SELECT * FROM "musicbrainz"."label_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_rating_raw: LabelRatingRaw) -> Result<LabelRatingRaw> {
        query_as::<_, LabelRatingRaw>(r#"INSERT INTO "label_rating_raw" ("label", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(label_rating_raw.rating)
            .bind(label_rating_raw.label)
            .bind(label_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_rating_raw: LabelRatingRaw) -> Result<LabelRatingRaw> {
        query_as::<_, LabelRatingRaw>(r#"UPDATE "label_rating_raw" SET "rating" = $3 WHERE "label" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(label_rating_raw.rating)
            .bind(label_rating_raw.label)
            .bind(label_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_rating_raw" WHERE "editor" = 2 AND "label" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
