#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaTag;

pub struct AreaTagSet;

impl AreaTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_area_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, tag: i32) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "area" = $1 AND "tag" = $2"#)
            .bind(area)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_area_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, area_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "area" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(area_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_area_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, tag: i32) -> Result<Option<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "area" = $1 AND "tag" = $2"#)
            .bind(area)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<AreaTag>> {
        query_as::<_, AreaTag>(r#"SELECT * FROM "musicbrainz"."area_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_tag: AreaTag) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"INSERT INTO "area_tag" ("area", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(area_tag.area)
            .bind(area_tag.tag)
            .bind(area_tag.count)
            .bind(area_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_tag: AreaTag) -> Result<AreaTag> {
        query_as::<_, AreaTag>(r#"UPDATE "area_tag" SET "count" = $3, "last_updated" = $4 WHERE "area" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(area_tag.area)
            .bind(area_tag.tag)
            .bind(area_tag.count)
            .bind(area_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_tag" WHERE "area" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
