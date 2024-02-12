#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkTag;

pub struct WorkTagSet;

impl WorkTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, tag: i32) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = $1 AND "tag" = $2"#)
            .bind(work)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_work_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(work_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, tag: i32) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "work" = $1 AND "tag" = $2"#)
            .bind(work)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<WorkTag>> {
        query_as::<_, WorkTag>(r#"SELECT * FROM "musicbrainz"."work_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag: WorkTag) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"INSERT INTO "work_tag" ("work", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(work_tag.work)
            .bind(work_tag.tag)
            .bind(work_tag.count)
            .bind(work_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_tag: WorkTag) -> Result<WorkTag> {
        query_as::<_, WorkTag>(r#"UPDATE "work_tag" SET "count" = $3, "last_updated" = $4 WHERE "work" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(work_tag.work)
            .bind(work_tag.tag)
            .bind(work_tag.count)
            .bind(work_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_tag" WHERE "work" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
