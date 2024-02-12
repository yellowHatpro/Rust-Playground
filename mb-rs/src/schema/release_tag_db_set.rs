#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseTag;

pub struct ReleaseTagSet;

impl ReleaseTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, tag: i32) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "release" = $1 AND "tag" = $2"#)
            .bind(release)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "release" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(release_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, tag: i32) -> Result<Option<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "release" = $1 AND "tag" = $2"#)
            .bind(release)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<ReleaseTag>> {
        query_as::<_, ReleaseTag>(r#"SELECT * FROM "musicbrainz"."release_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_tag: ReleaseTag) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"INSERT INTO "release_tag" ("release", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(release_tag.release)
            .bind(release_tag.tag)
            .bind(release_tag.count)
            .bind(release_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_tag: ReleaseTag) -> Result<ReleaseTag> {
        query_as::<_, ReleaseTag>(r#"UPDATE "release_tag" SET "count" = $3, "last_updated" = $4 WHERE "release" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(release_tag.release)
            .bind(release_tag.tag)
            .bind(release_tag.count)
            .bind(release_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_tag" WHERE "release" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
