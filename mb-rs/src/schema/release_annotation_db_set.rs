#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseAnnotation;

pub struct ReleaseAnnotationSet;

impl ReleaseAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, annotation: i32) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "release" = $1 AND "annotation" = $2"#)
            .bind(release)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "release" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(release_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32, annotation: i32) -> Result<Option<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "release" = $1 AND "annotation" = $2"#)
            .bind(release)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<ReleaseAnnotation>> {
        query_as::<_, ReleaseAnnotation>(r#"SELECT * FROM "musicbrainz"."release_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_annotation: ReleaseAnnotation) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"INSERT INTO "release_annotation" ("release", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(release_annotation.release)
            .bind(release_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_annotation: ReleaseAnnotation) -> Result<ReleaseAnnotation> {
        query_as::<_, ReleaseAnnotation>(r#"UPDATE "release_annotation" SET  WHERE "release" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(release_annotation.release)
            .bind(release_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_annotation" WHERE "release" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
