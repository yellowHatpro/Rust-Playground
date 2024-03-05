#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeLabelDeleted;

pub struct EditorSubscribeLabelDeletedSet;

impl EditorSubscribeLabelDeletedSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_editor_and_gid<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, gid: uuid::Uuid) -> Result<EditorSubscribeLabelDeleted> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE "editor" = $1 AND "gid" = $2"#)
            .bind(editor)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_editor_and_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, editor_list: Vec<i32>, gid_list: Vec<uuid::Uuid>) -> Result<Vec<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE "editor" = ANY($1) AND "gid" = ANY($2)"#)
            .bind(editor_list)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_editor_and_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, gid: uuid::Uuid) -> Result<Option<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE "editor" = $1 AND "gid" = $2"#)
            .bind(editor)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_deleted_entity_gid_where_gid_is<'e, E: PgExecutor<'e>>(executor: E, deleted_entity_gid: uuid::Uuid) -> Result<Vec<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE gid = $1"#)
            .bind(deleted_entity_gid)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id_where_deleted_by_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditorSubscribeLabelDeleted>> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE deleted_by = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_label_deleted: EditorSubscribeLabelDeleted) -> Result<EditorSubscribeLabelDeleted> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"INSERT INTO "editor_subscribe_label_deleted" ("editor", "gid", "deleted_by") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(editor_subscribe_label_deleted.deleted_by)
            .bind(editor_subscribe_label_deleted.editor)
            .bind(editor_subscribe_label_deleted.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_label_deleted: EditorSubscribeLabelDeleted) -> Result<EditorSubscribeLabelDeleted> {
        query_as::<_, EditorSubscribeLabelDeleted>(r#"UPDATE "editor_subscribe_label_deleted" SET "deleted_by" = $3 WHERE "gid" = 2 AND "editor" = 1 RETURNING *;"#)
            .bind(editor_subscribe_label_deleted.editor)
            .bind(editor_subscribe_label_deleted.gid)
            .bind(editor_subscribe_label_deleted.deleted_by)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_label_deleted" WHERE "editor" = 1 AND "gid" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
