#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CountryArea;

pub struct CountryAreaSet;

impl CountryAreaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_area<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "area" = $1"#)
            .bind(area)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_area_list<'e, E: PgExecutor<'e>>(&self, executor: E, area_list: Vec<i32>) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "area" = ANY($1)"#)
            .bind(area_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_area_optional<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32) -> Result<Option<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "area" = $1"#)
            .bind(area)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<CountryArea>> {
        query_as::<_, CountryArea>(r#"SELECT * FROM "musicbrainz"."country_area" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, country_area: CountryArea) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"INSERT INTO "country_area" ("area") VALUES ($1) RETURNING *;"#)
            .bind(country_area.area)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, country_area: CountryArea) -> Result<CountryArea> {
        query_as::<_, CountryArea>(r#"UPDATE "country_area" SET  WHERE "area" = 1 RETURNING *;"#)
            .bind(country_area.area)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."country_area" WHERE "area" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
