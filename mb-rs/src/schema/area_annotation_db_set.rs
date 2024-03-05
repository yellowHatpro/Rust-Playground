#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAnnotation;

pub struct AreaAnnotationSet;

impl AreaAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAnnotation>> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_area_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, annotation: i32) -> Result<AreaAnnotation> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation" WHERE "area" = $1 AND "annotation" = $2"#)
            .bind(area)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_area_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, area_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<AreaAnnotation>> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation" WHERE "area" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(area_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_area_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, annotation: i32) -> Result<Option<AreaAnnotation>> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation" WHERE "area" = $1 AND "annotation" = $2"#)
            .bind(area)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaAnnotation>> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<AreaAnnotation>> {
        query_as::<_, AreaAnnotation>(r#"SELECT * FROM "musicbrainz"."area_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_annotation: AreaAnnotation) -> Result<AreaAnnotation> {
        query_as::<_, AreaAnnotation>(r#"INSERT INTO "area_annotation" ("area", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(area_annotation.area)
            .bind(area_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_annotation: AreaAnnotation) -> Result<AreaAnnotation> {
        query_as::<_, AreaAnnotation>(r#"UPDATE "area_annotation" SET  WHERE "annotation" = 2 AND "area" = 1 RETURNING *;"#)
            .bind(area_annotation.area)
            .bind(area_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_annotation" WHERE "area" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
