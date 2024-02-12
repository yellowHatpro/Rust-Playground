#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditLabel;

pub struct EditLabelSet;

impl EditLabelSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_label<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, label: i32) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "edit" = $1 AND "label" = $2"#)
            .bind(edit)
            .bind(label)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_label_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, label_list: Vec<i32>) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "edit" = ANY($1) AND "label" = ANY($2)"#)
            .bind(edit_list)
            .bind(label_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_label_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, label: i32) -> Result<Option<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "edit" = $1 AND "label" = $2"#)
            .bind(edit)
            .bind(label)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<EditLabel>> {
        query_as::<_, EditLabel>(r#"SELECT * FROM "musicbrainz"."edit_label" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_label: EditLabel) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"INSERT INTO "edit_label" ("edit", "label", "status") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(edit_label.edit)
            .bind(edit_label.label)
            .bind(edit_label.status)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_label: EditLabel) -> Result<EditLabel> {
        query_as::<_, EditLabel>(r#"UPDATE "edit_label" SET "status" = $3 WHERE "edit" = 1 AND "label" = 2 RETURNING *;"#)
            .bind(edit_label.edit)
            .bind(edit_label.label)
            .bind(edit_label.status)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_label" WHERE "edit" = 1 AND "label" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
