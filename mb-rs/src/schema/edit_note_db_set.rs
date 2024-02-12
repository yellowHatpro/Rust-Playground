#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditNote;

pub struct EditNoteSet;

impl EditNoteSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note: EditNote) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"INSERT INTO "edit_note" ("id", "editor", "edit", "text", "post_time") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(edit_note.id)
            .bind(edit_note.editor)
            .bind(edit_note.edit)
            .bind(edit_note.text)
            .bind(edit_note.post_time)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note: EditNote) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"UPDATE "edit_note" SET "editor" = $2, "edit" = $3, "text" = $4, "post_time" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(edit_note.id)
            .bind(edit_note.editor)
            .bind(edit_note.edit)
            .bind(edit_note.text)
            .bind(edit_note.post_time)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_note" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
