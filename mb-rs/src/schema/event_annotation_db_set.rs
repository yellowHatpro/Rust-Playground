#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventAnnotation;

pub struct EventAnnotationSet;

impl EventAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_event_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, annotation: i32) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = $1 AND "annotation" = $2"#)
            .bind(event)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_event_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, event_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(event_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_event_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, event: i32, annotation: i32) -> Result<Option<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE "event" = $1 AND "annotation" = $2"#)
            .bind(event)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<EventAnnotation>> {
        query_as::<_, EventAnnotation>(r#"SELECT * FROM "musicbrainz"."event_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_annotation: EventAnnotation) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"INSERT INTO "event_annotation" ("event", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(event_annotation.annotation)
            .bind(event_annotation.event)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_annotation: EventAnnotation) -> Result<EventAnnotation> {
        query_as::<_, EventAnnotation>(r#"UPDATE "event_annotation" SET  WHERE "event" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(event_annotation.event)
            .bind(event_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_annotation" WHERE "event" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
