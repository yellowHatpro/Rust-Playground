#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceTag;

pub struct PlaceTagSet;

impl PlaceTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_place_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, tag: i32) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = $1 AND "tag" = $2"#)
            .bind(place)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_place_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, place_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(place_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_place_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, tag: i32) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = $1 AND "tag" = $2"#)
            .bind(place)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_place_id<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag: PlaceTag) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"INSERT INTO "place_tag" ("place", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(place_tag.place)
            .bind(place_tag.tag)
            .bind(place_tag.count)
            .bind(place_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag: PlaceTag) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"UPDATE "place_tag" SET "count" = $3, "last_updated" = $4 WHERE "place" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(place_tag.place)
            .bind(place_tag.tag)
            .bind(place_tag.count)
            .bind(place_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_tag" WHERE "place" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
