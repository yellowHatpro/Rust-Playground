#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Iso31661;

pub struct Iso31661Set;

impl Iso31661Set {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Iso31661>> {
        query_as::<_, Iso31661>(r#"SELECT * FROM "musicbrainz"."iso_3166_1""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_code<'e, E: PgExecutor<'e>>(&self, executor: E, code: String) -> Result<Iso31661> {
        query_as::<_, Iso31661>(r#"SELECT * FROM "musicbrainz"."iso_3166_1" WHERE "code" = $1"#)
            .bind(code)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_code_list<'e, E: PgExecutor<'e>>(&self, executor: E, code_list: Vec<String>) -> Result<Vec<Iso31661>> {
        query_as::<_, Iso31661>(r#"SELECT * FROM "musicbrainz"."iso_3166_1" WHERE "code" = ANY($1)"#)
            .bind(code_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_code_optional<'e, E: PgExecutor<'e>>(&self, executor: E, code: String) -> Result<Option<Iso31661>> {
        query_as::<_, Iso31661>(r#"SELECT * FROM "musicbrainz"."iso_3166_1" WHERE "code" = $1"#)
            .bind(code)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Iso31661>> {
        query_as::<_, Iso31661>(r#"SELECT * FROM "musicbrainz"."iso_3166_1" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, iso_3166_1: Iso31661) -> Result<Iso31661> {
        query_as::<_, Iso31661>(r#"INSERT INTO "iso_3166_1" ("area", "code") VALUES ($1, $2) RETURNING *;"#)
            .bind(iso_3166_1.code)
            .bind(iso_3166_1.area)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, iso_3166_1: Iso31661) -> Result<Iso31661> {
        query_as::<_, Iso31661>(r#"UPDATE "iso_3166_1" SET "area" = $1 WHERE "code" = 2 RETURNING *;"#)
            .bind(iso_3166_1.area)
            .bind(iso_3166_1.code)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."iso_3166_1" WHERE "code" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
