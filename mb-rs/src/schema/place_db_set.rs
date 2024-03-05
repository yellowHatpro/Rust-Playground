#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Place;

pub struct PlaceSet;

impl PlaceSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, place_type_id: i32) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE type = $1"#)
            .bind(place_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place: Place) -> Result<Place> {
        query_as::<_, Place>(r#"INSERT INTO "place" ("id", "gid", "name", "type", "address", "area", "coordinates", "comment", "edits_pending", "last_updated", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(place.address)
            .bind(place.area)
            .bind(place.begin_date_day)
            .bind(place.end_date_year)
            .bind(place.comment)
            .bind(place.edits_pending)
            .bind(place.coordinates)
            .bind(place.last_updated)
            .bind(place.end_date_month)
            .bind(place.name)
            .bind(place.end_date_day)
            .bind(place.ended)
            .bind(place.begin_date_month)
            .bind(place.gid)
            .bind(place.id)
            .bind(place.Type)
            .bind(place.begin_date_year)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place: Place) -> Result<Place> {
        query_as::<_, Place>(r#"UPDATE "place" SET "begin_date_month" = $12, "last_updated" = $10, "name" = $3, "coordinates" = $7, "ended" = $17, "end_date_month" = $15, "edits_pending" = $9, "area" = $6, "comment" = $8, "end_date_year" = $14, "address" = $5, "type" = $4, "begin_date_day" = $13, "end_date_day" = $16, "gid" = $2, "begin_date_year" = $11 WHERE "id" = 1 RETURNING *;"#)
            .bind(place.begin_date_day)
            .bind(place.end_date_day)
            .bind(place.coordinates)
            .bind(place.end_date_month)
            .bind(place.name)
            .bind(place.end_date_year)
            .bind(place.gid)
            .bind(place.begin_date_month)
            .bind(place.id)
            .bind(place.Type)
            .bind(place.area)
            .bind(place.ended)
            .bind(place.begin_date_year)
            .bind(place.comment)
            .bind(place.address)
            .bind(place.edits_pending)
            .bind(place.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
