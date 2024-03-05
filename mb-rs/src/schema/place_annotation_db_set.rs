#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceAnnotation;

pub struct PlaceAnnotationSet;

impl PlaceAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceAnnotation>> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_place_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, annotation: i32) -> Result<PlaceAnnotation> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation" WHERE "place" = $1 AND "annotation" = $2"#)
            .bind(place)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_place_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, place_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<PlaceAnnotation>> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation" WHERE "place" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(place_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_place_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, annotation: i32) -> Result<Option<PlaceAnnotation>> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation" WHERE "place" = $1 AND "annotation" = $2"#)
            .bind(place)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_id_where_place_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceAnnotation>> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<PlaceAnnotation>> {
        query_as::<_, PlaceAnnotation>(r#"SELECT * FROM "musicbrainz"."place_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_annotation: PlaceAnnotation) -> Result<PlaceAnnotation> {
        query_as::<_, PlaceAnnotation>(r#"INSERT INTO "place_annotation" ("place", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(place_annotation.place)
            .bind(place_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_annotation: PlaceAnnotation) -> Result<PlaceAnnotation> {
        query_as::<_, PlaceAnnotation>(r#"UPDATE "place_annotation" SET  WHERE "place" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(place_annotation.place)
            .bind(place_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_annotation" WHERE "annotation" = 2 AND "place" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
