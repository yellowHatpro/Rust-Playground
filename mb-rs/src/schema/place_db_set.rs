#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Place;

pub struct PlaceSet;

impl PlaceSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Place> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_place_type_id<'e, E: PgExecutor<'e>>(executor: E, place_type_id: i32) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE type = $1"#)
            .bind(place_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Place>> {
        query_as::<_, Place>(r#"SELECT * FROM "musicbrainz"."place" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place: Place) -> Result<Place> {
        query_as::<_, Place>(r#"INSERT INTO "place" ("id", "gid", "name", "type", "address", "area", "coordinates", "comment", "edits_pending", "last_updated", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(place.id)
            .bind(place.gid)
            .bind(place.name)
            .bind(place.Type)
            .bind(place.address)
            .bind(place.area)
            .bind(place.coordinates)
            .bind(place.comment)
            .bind(place.edits_pending)
            .bind(place.last_updated)
            .bind(place.begin_date_year)
            .bind(place.begin_date_month)
            .bind(place.begin_date_day)
            .bind(place.end_date_year)
            .bind(place.end_date_month)
            .bind(place.end_date_day)
            .bind(place.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place: Place) -> Result<Place> {
        query_as::<_, Place>(r#"UPDATE "place" SET "gid" = $2, "name" = $3, "type" = $4, "address" = $5, "area" = $6, "coordinates" = $7, "comment" = $8, "edits_pending" = $9, "last_updated" = $10, "begin_date_year" = $11, "begin_date_month" = $12, "begin_date_day" = $13, "end_date_year" = $14, "end_date_month" = $15, "end_date_day" = $16, "ended" = $17 WHERE "id" = 1 RETURNING *;"#)
            .bind(place.id)
            .bind(place.gid)
            .bind(place.name)
            .bind(place.Type)
            .bind(place.address)
            .bind(place.area)
            .bind(place.coordinates)
            .bind(place.comment)
            .bind(place.edits_pending)
            .bind(place.last_updated)
            .bind(place.begin_date_year)
            .bind(place.begin_date_month)
            .bind(place.begin_date_day)
            .bind(place.end_date_year)
            .bind(place.end_date_month)
            .bind(place.end_date_day)
            .bind(place.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
