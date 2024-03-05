#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAnnotation;

pub struct ReleaseGroupAnnotationSet;

impl ReleaseGroupAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAnnotation>> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release_group_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, annotation: i32) -> Result<ReleaseGroupAnnotation> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation" WHERE "release_group" = $1 AND "annotation" = $2"#)
            .bind(release_group)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_group_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<ReleaseGroupAnnotation>> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation" WHERE "release_group" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(release_group_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_group_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, annotation: i32) -> Result<Option<ReleaseGroupAnnotation>> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation" WHERE "release_group" = $1 AND "annotation" = $2"#)
            .bind(release_group)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupAnnotation>> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<ReleaseGroupAnnotation>> {
        query_as::<_, ReleaseGroupAnnotation>(r#"SELECT * FROM "musicbrainz"."release_group_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_annotation: ReleaseGroupAnnotation) -> Result<ReleaseGroupAnnotation> {
        query_as::<_, ReleaseGroupAnnotation>(r#"INSERT INTO "release_group_annotation" ("release_group", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(release_group_annotation.release_group)
            .bind(release_group_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_annotation: ReleaseGroupAnnotation) -> Result<ReleaseGroupAnnotation> {
        query_as::<_, ReleaseGroupAnnotation>(r#"UPDATE "release_group_annotation" SET  WHERE "release_group" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(release_group_annotation.annotation)
            .bind(release_group_annotation.release_group)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_annotation" WHERE "annotation" = 2 AND "release_group" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
