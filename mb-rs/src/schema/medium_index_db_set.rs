#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumIndex;

pub struct MediumIndexSet;

impl MediumIndexSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, medium: i32) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "medium" = $1"#)
            .bind(medium)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, medium_list: Vec<i32>) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "medium" = ANY($1)"#)
            .bind(medium_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, medium: i32) -> Result<Option<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "medium" = $1"#)
            .bind(medium)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_medium_id<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<MediumIndex>> {
        query_as::<_, MediumIndex>(r#"SELECT * FROM "musicbrainz"."medium_index" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_index: MediumIndex) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"INSERT INTO "medium_index" ("medium", "toc") VALUES ($1, $2) RETURNING *;"#)
            .bind(medium_index.medium)
            .bind(medium_index.toc)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_index: MediumIndex) -> Result<MediumIndex> {
        query_as::<_, MediumIndex>(r#"UPDATE "medium_index" SET "toc" = $2 WHERE "medium" = 1 RETURNING *;"#)
            .bind(medium_index.medium)
            .bind(medium_index.toc)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_index" WHERE "medium" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
