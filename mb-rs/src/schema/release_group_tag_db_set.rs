#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupTag;

pub struct ReleaseGroupTagSet;

impl ReleaseGroupTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_group_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, tag: i32) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "release_group" = $1 AND "tag" = $2"#)
            .bind(release_group)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_group_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "release_group" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(release_group_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_group_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, tag: i32) -> Result<Option<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "release_group" = $1 AND "tag" = $2"#)
            .bind(release_group)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<ReleaseGroupTag>> {
        query_as::<_, ReleaseGroupTag>(r#"SELECT * FROM "musicbrainz"."release_group_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_tag: ReleaseGroupTag) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"INSERT INTO "release_group_tag" ("release_group", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(release_group_tag.release_group)
            .bind(release_group_tag.tag)
            .bind(release_group_tag.count)
            .bind(release_group_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_tag: ReleaseGroupTag) -> Result<ReleaseGroupTag> {
        query_as::<_, ReleaseGroupTag>(r#"UPDATE "release_group_tag" SET "count" = $3, "last_updated" = $4 WHERE "release_group" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(release_group_tag.release_group)
            .bind(release_group_tag.tag)
            .bind(release_group_tag.count)
            .bind(release_group_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_tag" WHERE "release_group" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
