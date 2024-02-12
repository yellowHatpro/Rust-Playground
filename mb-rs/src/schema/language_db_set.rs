#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Language;

pub struct LanguageSet;

impl LanguageSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Language> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Language> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Language> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Language> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Language> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Language>> {
        query_as::<_, Language>(r#"SELECT * FROM "musicbrainz"."language" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, language: Language) -> Result<Language> {
        query_as::<_, Language>(r#"INSERT INTO "language" ("id", "iso_code_2t", "iso_code_2b", "iso_code_1", "name", "frequency", "iso_code_3") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(language.id)
            .bind(language.iso_code_2t)
            .bind(language.iso_code_2b)
            .bind(language.iso_code_1)
            .bind(language.name)
            .bind(language.frequency)
            .bind(language.iso_code_3)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, language: Language) -> Result<Language> {
        query_as::<_, Language>(r#"UPDATE "language" SET "iso_code_2t" = $2, "iso_code_2b" = $3, "iso_code_1" = $4, "name" = $5, "frequency" = $6, "iso_code_3" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(language.id)
            .bind(language.iso_code_2t)
            .bind(language.iso_code_2b)
            .bind(language.iso_code_1)
            .bind(language.name)
            .bind(language.frequency)
            .bind(language.iso_code_3)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."language" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
