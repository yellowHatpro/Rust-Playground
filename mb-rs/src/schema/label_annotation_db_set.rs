#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelAnnotation;

pub struct LabelAnnotationSet;

impl LabelAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelAnnotation>> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_label_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, annotation: i32) -> Result<LabelAnnotation> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation" WHERE "label" = $1 AND "annotation" = $2"#)
            .bind(label)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_label_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<LabelAnnotation>> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation" WHERE "label" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(label_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_label_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, annotation: i32) -> Result<Option<LabelAnnotation>> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation" WHERE "label" = $1 AND "annotation" = $2"#)
            .bind(label)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelAnnotation>> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<LabelAnnotation>> {
        query_as::<_, LabelAnnotation>(r#"SELECT * FROM "musicbrainz"."label_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_annotation: LabelAnnotation) -> Result<LabelAnnotation> {
        query_as::<_, LabelAnnotation>(r#"INSERT INTO "label_annotation" ("label", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(label_annotation.annotation)
            .bind(label_annotation.label)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_annotation: LabelAnnotation) -> Result<LabelAnnotation> {
        query_as::<_, LabelAnnotation>(r#"UPDATE "label_annotation" SET  WHERE "label" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(label_annotation.label)
            .bind(label_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_annotation" WHERE "label" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
