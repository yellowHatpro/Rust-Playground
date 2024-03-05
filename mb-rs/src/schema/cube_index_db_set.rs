#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CubeIndex;

pub struct CubeIndexSet;

impl CubeIndexSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, cube_index: CubeIndex) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"INSERT INTO "cube_index" ("medium", "toc") VALUES ($1, $2) RETURNING *;"#)
            .bind(cube_index.medium)
            .bind(cube_index.toc)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, cube_index: CubeIndex) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"UPDATE "cube_index" SET "toc" = $2, "medium" = $1 WHERE  RETURNING *;"#)
            .bind(cube_index.medium)
            .bind(cube_index.toc)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."cube_index" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
