#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorPreference;

pub struct EditorPreferenceSet;

impl EditorPreferenceSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorPreference>> {
        query_as::<_, EditorPreference>(r#"SELECT * FROM "musicbrainz"."editor_preference" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_preference: EditorPreference) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"INSERT INTO "editor_preference" ("id", "editor", "name", "value") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(editor_preference.id)
            .bind(editor_preference.editor)
            .bind(editor_preference.name)
            .bind(editor_preference.value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_preference: EditorPreference) -> Result<EditorPreference> {
        query_as::<_, EditorPreference>(r#"UPDATE "editor_preference" SET "editor" = $2, "name" = $3, "value" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_preference.id)
            .bind(editor_preference.editor)
            .bind(editor_preference.name)
            .bind(editor_preference.value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_preference" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
