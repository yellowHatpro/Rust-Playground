#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceRatingRaw;

pub struct PlaceRatingRawSet;

impl PlaceRatingRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceRatingRaw>> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_place_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, editor: i32) -> Result<PlaceRatingRaw> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw" WHERE "place" = $1 AND "editor" = $2"#)
            .bind(place)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_place_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, place_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<PlaceRatingRaw>> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw" WHERE "place" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(place_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_place_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, editor: i32) -> Result<Option<PlaceRatingRaw>> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw" WHERE "place" = $1 AND "editor" = $2"#)
            .bind(place)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_id_where_place_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceRatingRaw>> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<PlaceRatingRaw>> {
        query_as::<_, PlaceRatingRaw>(r#"SELECT * FROM "musicbrainz"."place_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_rating_raw: PlaceRatingRaw) -> Result<PlaceRatingRaw> {
        query_as::<_, PlaceRatingRaw>(r#"INSERT INTO "place_rating_raw" ("place", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(place_rating_raw.place)
            .bind(place_rating_raw.editor)
            .bind(place_rating_raw.rating)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_rating_raw: PlaceRatingRaw) -> Result<PlaceRatingRaw> {
        query_as::<_, PlaceRatingRaw>(r#"UPDATE "place_rating_raw" SET "rating" = $3 WHERE "editor" = 2 AND "place" = 1 RETURNING *;"#)
            .bind(place_rating_raw.place)
            .bind(place_rating_raw.rating)
            .bind(place_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_rating_raw" WHERE "place" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
