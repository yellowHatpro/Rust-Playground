#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseFirstReleaseDate;

pub struct ReleaseFirstReleaseDateSet;

impl ReleaseFirstReleaseDateSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "release" = $1"#)
            .bind(release)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "release" = ANY($1)"#)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<Option<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "release" = $1"#)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseFirstReleaseDate>> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"SELECT * FROM "musicbrainz"."release_first_release_date" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_first_release_date: ReleaseFirstReleaseDate) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"INSERT INTO "release_first_release_date" ("release", "year", "month", "day") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(release_first_release_date.release)
            .bind(release_first_release_date.year)
            .bind(release_first_release_date.month)
            .bind(release_first_release_date.day)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_first_release_date: ReleaseFirstReleaseDate) -> Result<ReleaseFirstReleaseDate> {
        query_as::<_, ReleaseFirstReleaseDate>(r#"UPDATE "release_first_release_date" SET "year" = $2, "month" = $3, "day" = $4 WHERE "release" = 1 RETURNING *;"#)
            .bind(release_first_release_date.release)
            .bind(release_first_release_date.year)
            .bind(release_first_release_date.month)
            .bind(release_first_release_date.day)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_first_release_date" WHERE "release" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
