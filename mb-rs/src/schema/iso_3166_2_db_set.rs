#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Iso31662;

pub struct Iso31662Set;

impl Iso31662Set {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_code<'e, E: PgExecutor<'e>>(&self, executor: E, code: String) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "code" = $1"#)
            .bind(code)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_code_list<'e, E: PgExecutor<'e>>(&self, executor: E, code_list: Vec<String>) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "code" = ANY($1)"#)
            .bind(code_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_code_optional<'e, E: PgExecutor<'e>>(&self, executor: E, code: String) -> Result<Option<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "code" = $1"#)
            .bind(code)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Iso31662>> {
        query_as::<_, Iso31662>(r#"SELECT * FROM "musicbrainz"."iso_3166_2" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, iso_3166_2: Iso31662) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"INSERT INTO "iso_3166_2" ("area", "code") VALUES ($1, $2) RETURNING *;"#)
            .bind(iso_3166_2.area)
            .bind(iso_3166_2.code)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, iso_3166_2: Iso31662) -> Result<Iso31662> {
        query_as::<_, Iso31662>(r#"UPDATE "iso_3166_2" SET "area" = $1 WHERE "code" = 2 RETURNING *;"#)
            .bind(iso_3166_2.area)
            .bind(iso_3166_2.code)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."iso_3166_2" WHERE "code" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
