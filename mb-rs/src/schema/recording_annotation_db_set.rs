#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingAnnotation;

pub struct RecordingAnnotationSet;

impl RecordingAnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingAnnotation>> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_recording_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, annotation: i32) -> Result<RecordingAnnotation> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation" WHERE "recording" = $1 AND "annotation" = $2"#)
            .bind(recording)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_recording_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, recording_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<RecordingAnnotation>> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation" WHERE "recording" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(recording_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_recording_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, recording: i32, annotation: i32) -> Result<Option<RecordingAnnotation>> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation" WHERE "recording" = $1 AND "annotation" = $2"#)
            .bind(recording)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingAnnotation>> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id_where_annotation_is<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<RecordingAnnotation>> {
        query_as::<_, RecordingAnnotation>(r#"SELECT * FROM "musicbrainz"."recording_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_annotation: RecordingAnnotation) -> Result<RecordingAnnotation> {
        query_as::<_, RecordingAnnotation>(r#"INSERT INTO "recording_annotation" ("recording", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(recording_annotation.recording)
            .bind(recording_annotation.annotation)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_annotation: RecordingAnnotation) -> Result<RecordingAnnotation> {
        query_as::<_, RecordingAnnotation>(r#"UPDATE "recording_annotation" SET  WHERE "annotation" = 2 AND "recording" = 1 RETURNING *;"#)
            .bind(recording_annotation.annotation)
            .bind(recording_annotation.recording)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_annotation" WHERE "annotation" = 2 AND "recording" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
