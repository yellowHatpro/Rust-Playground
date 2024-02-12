#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MoodAnnotation;

pub struct MoodAnnotationSet;

impl MoodAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_mood_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, mood: i32, annotation: i32) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "mood" = $1 AND "annotation" = $2"#)
            .bind(mood)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_mood_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, mood_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "mood" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(mood_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_mood_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, mood: i32, annotation: i32) -> Result<Option<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "mood" = $1 AND "annotation" = $2"#)
            .bind(mood)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_mood_id<'e, E: PgExecutor<'e>>(executor: E, mood_id: i32) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE mood = $1"#)
            .bind(mood_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<MoodAnnotation>> {
        query_as::<_, MoodAnnotation>(r#"SELECT * FROM "musicbrainz"."mood_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, mood_annotation: MoodAnnotation) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"INSERT INTO "mood_annotation" ("mood", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(mood_annotation.mood)
            .bind(mood_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, mood_annotation: MoodAnnotation) -> Result<MoodAnnotation> {
        query_as::<_, MoodAnnotation>(r#"UPDATE "mood_annotation" SET  WHERE "mood" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(mood_annotation.mood)
            .bind(mood_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."mood_annotation" WHERE "mood" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
