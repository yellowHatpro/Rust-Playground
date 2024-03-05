#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseUnknownCountry;

pub struct ReleaseUnknownCountrySet;

impl ReleaseUnknownCountrySet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseUnknownCountry>> {
        query_as::<_, ReleaseUnknownCountry>(r#"SELECT * FROM "musicbrainz"."release_unknown_country""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<ReleaseUnknownCountry> {
        query_as::<_, ReleaseUnknownCountry>(r#"SELECT * FROM "musicbrainz"."release_unknown_country" WHERE "release" = $1"#)
            .bind(release)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>) -> Result<Vec<ReleaseUnknownCountry>> {
        query_as::<_, ReleaseUnknownCountry>(r#"SELECT * FROM "musicbrainz"."release_unknown_country" WHERE "release" = ANY($1)"#)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<Option<ReleaseUnknownCountry>> {
        query_as::<_, ReleaseUnknownCountry>(r#"SELECT * FROM "musicbrainz"."release_unknown_country" WHERE "release" = $1"#)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseUnknownCountry>> {
        query_as::<_, ReleaseUnknownCountry>(r#"SELECT * FROM "musicbrainz"."release_unknown_country" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_unknown_country: ReleaseUnknownCountry) -> Result<ReleaseUnknownCountry> {
        query_as::<_, ReleaseUnknownCountry>(r#"INSERT INTO "release_unknown_country" ("release", "date_year", "date_month", "date_day") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(release_unknown_country.date_year)
            .bind(release_unknown_country.release)
            .bind(release_unknown_country.date_month)
            .bind(release_unknown_country.date_day)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_unknown_country: ReleaseUnknownCountry) -> Result<ReleaseUnknownCountry> {
        query_as::<_, ReleaseUnknownCountry>(r#"UPDATE "release_unknown_country" SET "date_year" = $2, "date_day" = $4, "date_month" = $3 WHERE "release" = 1 RETURNING *;"#)
            .bind(release_unknown_country.date_day)
            .bind(release_unknown_country.date_year)
            .bind(release_unknown_country.release)
            .bind(release_unknown_country.date_month)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_unknown_country" WHERE "release" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
