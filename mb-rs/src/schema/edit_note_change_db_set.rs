#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditNoteChange;

pub struct EditNoteChangeSet;

impl EditNoteChangeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_note_id<'e, E: PgExecutor<'e>>(executor: E, edit_note_id: i32) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE edit_note = $1"#)
            .bind(edit_note_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditNoteChange>> {
        query_as::<_, EditNoteChange>(r#"SELECT * FROM "musicbrainz"."edit_note_change" WHERE change_editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note_change: EditNoteChange) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"INSERT INTO "edit_note_change" ("id", "status", "edit_note", "change_editor", "change_time", "old_note", "new_note", "reason") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(edit_note_change.id)
            .bind(edit_note_change.status)
            .bind(edit_note_change.edit_note)
            .bind(edit_note_change.change_editor)
            .bind(edit_note_change.change_time)
            .bind(edit_note_change.old_note)
            .bind(edit_note_change.new_note)
            .bind(edit_note_change.reason)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note_change: EditNoteChange) -> Result<EditNoteChange> {
        query_as::<_, EditNoteChange>(r#"UPDATE "edit_note_change" SET "status" = $2, "edit_note" = $3, "change_editor" = $4, "change_time" = $5, "old_note" = $6, "new_note" = $7, "reason" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(edit_note_change.id)
            .bind(edit_note_change.status)
            .bind(edit_note_change.edit_note)
            .bind(edit_note_change.change_editor)
            .bind(edit_note_change.change_time)
            .bind(edit_note_change.old_note)
            .bind(edit_note_change.new_note)
            .bind(edit_note_change.reason)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_note_change" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
