#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeRelease;

pub struct AlternativeReleaseSet;

impl AlternativeReleaseSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AlternativeRelease> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_credit_id_where_artist_credit_is<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_alternative_release_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, alternative_release_type_id: i32) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE type = $1"#)
            .bind(alternative_release_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_language_id_where_language_is<'e, E: PgExecutor<'e>>(executor: E, language_id: i32) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE language = $1"#)
            .bind(language_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_script_id_where_script_is<'e, E: PgExecutor<'e>>(executor: E, script_id: i32) -> Result<Vec<AlternativeRelease>> {
        query_as::<_, AlternativeRelease>(r#"SELECT * FROM "musicbrainz"."alternative_release" WHERE script = $1"#)
            .bind(script_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_release: AlternativeRelease) -> Result<AlternativeRelease> {
        query_as::<_, AlternativeRelease>(r#"INSERT INTO "alternative_release" ("id", "gid", "release", "name", "artist_credit", "type", "language", "script", "comment") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(alternative_release.gid)
            .bind(alternative_release.id)
            .bind(alternative_release.language)
            .bind(alternative_release.name)
            .bind(alternative_release.artist_credit)
            .bind(alternative_release.script)
            .bind(alternative_release.release)
            .bind(alternative_release.comment)
            .bind(alternative_release.Type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_release: AlternativeRelease) -> Result<AlternativeRelease> {
        query_as::<_, AlternativeRelease>(r#"UPDATE "alternative_release" SET "release" = $3, "language" = $7, "artist_credit" = $5, "comment" = $9, "gid" = $2, "script" = $8, "name" = $4, "type" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(alternative_release.id)
            .bind(alternative_release.comment)
            .bind(alternative_release.Type)
            .bind(alternative_release.gid)
            .bind(alternative_release.name)
            .bind(alternative_release.release)
            .bind(alternative_release.artist_credit)
            .bind(alternative_release.script)
            .bind(alternative_release.language)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_release" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
