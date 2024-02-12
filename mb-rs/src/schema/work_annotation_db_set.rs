#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkAnnotation;

pub struct WorkAnnotationSet;

impl WorkAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, annotation: i32) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "work" = $1 AND "annotation" = $2"#)
            .bind(work)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_work_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, work_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "work" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(work_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_work_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, work: i32, annotation: i32) -> Result<Option<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "work" = $1 AND "annotation" = $2"#)
            .bind(work)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<WorkAnnotation>> {
        query_as::<_, WorkAnnotation>(r#"SELECT * FROM "musicbrainz"."work_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_annotation: WorkAnnotation) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"INSERT INTO "work_annotation" ("work", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(work_annotation.work)
            .bind(work_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_annotation: WorkAnnotation) -> Result<WorkAnnotation> {
        query_as::<_, WorkAnnotation>(r#"UPDATE "work_annotation" SET  WHERE "work" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(work_annotation.work)
            .bind(work_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_annotation" WHERE "work" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
