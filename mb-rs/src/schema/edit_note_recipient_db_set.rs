#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditNoteRecipient;

pub struct EditNoteRecipientSet;

impl EditNoteRecipientSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recipient_and_edit_note<'e, E: PgExecutor<'e>>(&self, executor: E, recipient: i32, edit_note: i32) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "recipient" = $1 AND "edit_note" = $2"#)
            .bind(recipient)
            .bind(edit_note)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_recipient_and_edit_note_list<'e, E: PgExecutor<'e>>(&self, executor: E, recipient_list: Vec<i32>, edit_note_list: Vec<i32>) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "recipient" = ANY($1) AND "edit_note" = ANY($2)"#)
            .bind(recipient_list)
            .bind(edit_note_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_recipient_and_edit_note_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recipient: i32, edit_note: i32) -> Result<Option<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "recipient" = $1 AND "edit_note" = $2"#)
            .bind(recipient)
            .bind(edit_note)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE recipient = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_note_id<'e, E: PgExecutor<'e>>(executor: E, edit_note_id: i32) -> Result<Vec<EditNoteRecipient>> {
        query_as::<_, EditNoteRecipient>(r#"SELECT * FROM "musicbrainz"."edit_note_recipient" WHERE edit_note = $1"#)
            .bind(edit_note_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note_recipient: EditNoteRecipient) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"INSERT INTO "edit_note_recipient" ("recipient", "edit_note") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_note_recipient.recipient)
            .bind(edit_note_recipient.edit_note)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_note_recipient: EditNoteRecipient) -> Result<EditNoteRecipient> {
        query_as::<_, EditNoteRecipient>(r#"UPDATE "edit_note_recipient" SET  WHERE "recipient" = 1 AND "edit_note" = 2 RETURNING *;"#)
            .bind(edit_note_recipient.recipient)
            .bind(edit_note_recipient.edit_note)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_note_recipient" WHERE "recipient" = 1 AND "edit_note" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
