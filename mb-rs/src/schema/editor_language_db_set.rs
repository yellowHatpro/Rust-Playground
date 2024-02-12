#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorLanguage;

pub struct EditorLanguageSet;

impl EditorLanguageSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_editor_and_language<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, language: i32) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "editor" = $1 AND "language" = $2"#)
            .bind(editor)
            .bind(language)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_editor_and_language_list<'e, E: PgExecutor<'e>>(&self, executor: E, editor_list: Vec<i32>, language_list: Vec<i32>) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "editor" = ANY($1) AND "language" = ANY($2)"#)
            .bind(editor_list)
            .bind(language_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_editor_and_language_optional<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, language: i32) -> Result<Option<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "editor" = $1 AND "language" = $2"#)
            .bind(editor)
            .bind(language)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_language_id<'e, E: PgExecutor<'e>>(executor: E, language_id: i32) -> Result<Vec<EditorLanguage>> {
        query_as::<_, EditorLanguage>(r#"SELECT * FROM "musicbrainz"."editor_language" WHERE language = $1"#)
            .bind(language_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_language: EditorLanguage) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"INSERT INTO "editor_language" ("editor", "language", "fluency") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(editor_language.editor)
            .bind(editor_language.language)
            .bind(editor_language.fluency)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_language: EditorLanguage) -> Result<EditorLanguage> {
        query_as::<_, EditorLanguage>(r#"UPDATE "editor_language" SET "fluency" = $3 WHERE "editor" = 1 AND "language" = 2 RETURNING *;"#)
            .bind(editor_language.editor)
            .bind(editor_language.language)
            .bind(editor_language.fluency)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_language" WHERE "editor" = 1 AND "language" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
