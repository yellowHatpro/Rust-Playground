#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelTagRaw;

pub struct LabelTagRawSet;

impl LabelTagRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_label_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, editor: i32, tag: i32) -> Result<LabelTagRaw> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE "label" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(label)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_label_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE "label" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(label_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_label_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, editor: i32, tag: i32) -> Result<Option<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE "label" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(label)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<LabelTagRaw>> {
        query_as::<_, LabelTagRaw>(r#"SELECT * FROM "musicbrainz"."label_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_tag_raw: LabelTagRaw) -> Result<LabelTagRaw> {
        query_as::<_, LabelTagRaw>(r#"INSERT INTO "label_tag_raw" ("label", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_tag_raw.is_upvote)
            .bind(label_tag_raw.label)
            .bind(label_tag_raw.editor)
            .bind(label_tag_raw.tag)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_tag_raw: LabelTagRaw) -> Result<LabelTagRaw> {
        query_as::<_, LabelTagRaw>(r#"UPDATE "label_tag_raw" SET "is_upvote" = $4 WHERE "tag" = 3 AND "label" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(label_tag_raw.editor)
            .bind(label_tag_raw.tag)
            .bind(label_tag_raw.is_upvote)
            .bind(label_tag_raw.label)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_tag_raw" WHERE "label" = 1 AND "editor" = 2 AND "tag" = 3"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
