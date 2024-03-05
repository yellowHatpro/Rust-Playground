#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceTag;

pub struct PlaceTagSet;

impl PlaceTagSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_place_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, tag: i32) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = $1 AND "tag" = $2"#)
            .bind(place)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_place_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, place_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(place_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_place_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, tag: i32) -> Result<Option<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE "place" = $1 AND "tag" = $2"#)
            .bind(place)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_id_where_place_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<PlaceTag>> {
        query_as::<_, PlaceTag>(r#"SELECT * FROM "musicbrainz"."place_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag: PlaceTag) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"INSERT INTO "place_tag" ("place", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(place_tag.count)
            .bind(place_tag.last_updated)
            .bind(place_tag.place)
            .bind(place_tag.tag)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag: PlaceTag) -> Result<PlaceTag> {
        query_as::<_, PlaceTag>(r#"UPDATE "place_tag" SET "count" = $3, "last_updated" = $4 WHERE "place" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(place_tag.tag)
            .bind(place_tag.place)
            .bind(place_tag.count)
            .bind(place_tag.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_tag" WHERE "place" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
