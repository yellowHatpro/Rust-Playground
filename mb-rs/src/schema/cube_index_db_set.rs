#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CubeIndex;

pub struct CubeIndexSet;

impl CubeIndexSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CubeIndex>> {
        query_as::<_, CubeIndex>(r#"SELECT * FROM "musicbrainz"."cube_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, cube_index: CubeIndex) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"INSERT INTO "cube_index" ("medium", "toc") VALUES ($1, $2) RETURNING *;"#)
            .bind(cube_index.medium)
            .bind(cube_index.toc)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, cube_index: CubeIndex) -> Result<CubeIndex> {
        query_as::<_, CubeIndex>(r#"UPDATE "cube_index" SET "medium" = $1, "toc" = $2 WHERE  RETURNING *;"#)
            .bind(cube_index.medium)
            .bind(cube_index.toc)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."cube_index" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
