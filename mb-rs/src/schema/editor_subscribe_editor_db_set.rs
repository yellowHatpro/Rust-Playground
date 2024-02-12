#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeEditor;

pub struct EditorSubscribeEditorSet;

impl EditorSubscribeEditorSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeEditor>> {
        query_as::<_, EditorSubscribeEditor>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_editor" WHERE subscribed_editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_editor: EditorSubscribeEditor) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"INSERT INTO "editor_subscribe_editor" ("id", "editor", "subscribed_editor", "last_edit_sent") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(editor_subscribe_editor.id)
            .bind(editor_subscribe_editor.editor)
            .bind(editor_subscribe_editor.subscribed_editor)
            .bind(editor_subscribe_editor.last_edit_sent)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_editor: EditorSubscribeEditor) -> Result<EditorSubscribeEditor> {
        query_as::<_, EditorSubscribeEditor>(r#"UPDATE "editor_subscribe_editor" SET "editor" = $2, "subscribed_editor" = $3, "last_edit_sent" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_subscribe_editor.id)
            .bind(editor_subscribe_editor.editor)
            .bind(editor_subscribe_editor.subscribed_editor)
            .bind(editor_subscribe_editor.last_edit_sent)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_editor" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
