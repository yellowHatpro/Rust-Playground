#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeLabel;

pub struct EditorSubscribeLabelSet;

impl EditorSubscribeLabelSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorSubscribeLabel> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id_where_last_edit_sent_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditorSubscribeLabel>> {
        query_as::<_, EditorSubscribeLabel>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label" WHERE last_edit_sent = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_label: EditorSubscribeLabel) -> Result<EditorSubscribeLabel> {
        query_as::<_, EditorSubscribeLabel>(r#"INSERT INTO "editor_subscribe_label" ("id", "editor", "label", "last_edit_sent") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(editor_subscribe_label.label)
            .bind(editor_subscribe_label.editor)
            .bind(editor_subscribe_label.last_edit_sent)
            .bind(editor_subscribe_label.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_label: EditorSubscribeLabel) -> Result<EditorSubscribeLabel> {
        query_as::<_, EditorSubscribeLabel>(r#"UPDATE "editor_subscribe_label" SET "label" = $3, "last_edit_sent" = $4, "editor" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_subscribe_label.editor)
            .bind(editor_subscribe_label.id)
            .bind(editor_subscribe_label.last_edit_sent)
            .bind(editor_subscribe_label.label)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_label" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
