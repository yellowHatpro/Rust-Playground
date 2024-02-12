#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventAnnotation;

pub struct EventAnnotationSet;

impl EventAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, annotation: i32) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = $1 AND "annotation" = $2"#)
            .bind(event)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_event_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(event_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_event_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, annotation: i32) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = $1 AND "annotation" = $2"#)
            .bind(event)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_annotation: EventAnnotation) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"INSERT INTO "event_annotation" ("event", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(event_annotation.event)
            .bind(event_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_annotation: EventAnnotation) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"UPDATE "event_annotation" SET  WHERE "event" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(event_annotation.event)
            .bind(event_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_annotation" WHERE "event" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
