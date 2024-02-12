#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AlternativeMedium;

pub struct AlternativeMediumSet;

impl AlternativeMediumSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_medium_id<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_alternative_release_id<'e, E: PgExecutor<'e>>(executor: E, alternative_release_id: i32) -> Result<Vec<AlternativeMedium>> {
        query_as::<_, AlternativeMedium>(r#"SELECT * FROM "musicbrainz"."alternative_medium" WHERE alternative_release = $1"#)
            .bind(alternative_release_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: AlternativeMedium) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"INSERT INTO "alternative_medium" ("id", "medium", "alternative_release", "name") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(alternative_medium.id)
            .bind(alternative_medium.medium)
            .bind(alternative_medium.alternative_release)
            .bind(alternative_medium.name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, alternative_medium: AlternativeMedium) -> Result<AlternativeMedium> {
        query_as::<_, AlternativeMedium>(r#"UPDATE "alternative_medium" SET "medium" = $2, "alternative_release" = $3, "name" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(alternative_medium.id)
            .bind(alternative_medium.medium)
            .bind(alternative_medium.alternative_release)
            .bind(alternative_medium.name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."alternative_medium" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
