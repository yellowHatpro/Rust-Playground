#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseCountry;

pub struct ReleaseCountrySet;

impl ReleaseCountrySet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseCountry>> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release_and_country<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, country: i32) -> Result<ReleaseCountry> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country" WHERE "release" = $1 AND "country" = $2"#)
            .bind(release)
            .bind(country)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_and_country_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>, country_list: Vec<i32>) -> Result<Vec<ReleaseCountry>> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country" WHERE "release" = ANY($1) AND "country" = ANY($2)"#)
            .bind(release_list)
            .bind(country_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_and_country_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, country: i32) -> Result<Option<ReleaseCountry>> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country" WHERE "release" = $1 AND "country" = $2"#)
            .bind(release)
            .bind(country)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseCountry>> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_country_area_area_where_country_is<'e, E: PgExecutor<'e>>(executor: E, country_area_area: i32) -> Result<Vec<ReleaseCountry>> {
        query_as::<_, ReleaseCountry>(r#"SELECT * FROM "musicbrainz"."release_country" WHERE country = $1"#)
            .bind(country_area_area)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_country: ReleaseCountry) -> Result<ReleaseCountry> {
        query_as::<_, ReleaseCountry>(r#"INSERT INTO "release_country" ("release", "country", "date_year", "date_month", "date_day") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_country.date_year)
            .bind(release_country.date_day)
            .bind(release_country.release)
            .bind(release_country.country)
            .bind(release_country.date_month)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_country: ReleaseCountry) -> Result<ReleaseCountry> {
        query_as::<_, ReleaseCountry>(r#"UPDATE "release_country" SET "date_month" = $4, "date_day" = $5, "date_year" = $3 WHERE "country" = 2 AND "release" = 1 RETURNING *;"#)
            .bind(release_country.country)
            .bind(release_country.date_month)
            .bind(release_country.release)
            .bind(release_country.date_year)
            .bind(release_country.date_day)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_country" WHERE "release" = 1 AND "country" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
