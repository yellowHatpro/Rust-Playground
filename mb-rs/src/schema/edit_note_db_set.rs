#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditNote;

pub struct EditNoteSet;

impl EditNoteSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditNote>> {
        query_as::<_, EditNote>(r#"SELECT * FROM "musicbrainz"."edit_note" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note: EditNote) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"INSERT INTO "edit_note" ("id", "editor", "edit", "text", "post_time") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(edit_note.post_time)
            .bind(edit_note.text)
            .bind(edit_note.edit)
            .bind(edit_note.id)
            .bind(edit_note.editor)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note: EditNote) -> Result<EditNote> {
        query_as::<_, EditNote>(r#"UPDATE "edit_note" SET "post_time" = $5, "edit" = $3, "text" = $4, "editor" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(edit_note.post_time)
            .bind(edit_note.edit)
            .bind(edit_note.editor)
            .bind(edit_note.text)
            .bind(edit_note.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_note" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
