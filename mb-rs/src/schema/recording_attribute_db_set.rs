#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingAttribute;

pub struct RecordingAttributeSet;

impl RecordingAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<RecordingAttribute> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_attribute_type_id_where_recording_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, recording_attribute_type_id: i32) -> Result<Vec<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE recording_attribute_type = $1"#)
            .bind(recording_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_attribute_type_allowed_value_id_where_recording_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, recording_attribute_type_allowed_value_id: i32) -> Result<Vec<RecordingAttribute>> {
        query_as::<_, RecordingAttribute>(r#"SELECT * FROM "musicbrainz"."recording_attribute" WHERE recording_attribute_type_allowed_value = $1"#)
            .bind(recording_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_attribute: RecordingAttribute) -> Result<RecordingAttribute> {
        query_as::<_, RecordingAttribute>(r#"INSERT INTO "recording_attribute" ("id", "recording", "recording_attribute_type", "recording_attribute_type_allowed_value", "recording_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(recording_attribute.recording_attribute_text)
            .bind(recording_attribute.recording)
            .bind(recording_attribute.recording_attribute_type_allowed_value)
            .bind(recording_attribute.id)
            .bind(recording_attribute.recording_attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_attribute: RecordingAttribute) -> Result<RecordingAttribute> {
        query_as::<_, RecordingAttribute>(r#"UPDATE "recording_attribute" SET "recording_attribute_text" = $5, "recording_attribute_type_allowed_value" = $4, "recording" = $2, "recording_attribute_type" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(recording_attribute.id)
            .bind(recording_attribute.recording_attribute_type_allowed_value)
            .bind(recording_attribute.recording_attribute_text)
            .bind(recording_attribute.recording)
            .bind(recording_attribute.recording_attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
