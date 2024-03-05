#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingAttributeType;

pub struct RecordingAttributeTypeSet;

impl RecordingAttributeTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingAttributeType>> {
        query_as::<_, RecordingAttributeType>(r#"SELECT * FROM "musicbrainz"."recording_attribute_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<RecordingAttributeType> {
        query_as::<_, RecordingAttributeType>(r#"SELECT * FROM "musicbrainz"."recording_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<RecordingAttributeType>> {
        query_as::<_, RecordingAttributeType>(r#"SELECT * FROM "musicbrainz"."recording_attribute_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<RecordingAttributeType>> {
        query_as::<_, RecordingAttributeType>(r#"SELECT * FROM "musicbrainz"."recording_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_attribute_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, recording_attribute_type_id: i32) -> Result<Vec<RecordingAttributeType>> {
        query_as::<_, RecordingAttributeType>(r#"SELECT * FROM "musicbrainz"."recording_attribute_type" WHERE parent = $1"#)
            .bind(recording_attribute_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_attribute_type: RecordingAttributeType) -> Result<RecordingAttributeType> {
        query_as::<_, RecordingAttributeType>(r#"INSERT INTO "recording_attribute_type" ("id", "name", "comment", "free_text", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(recording_attribute_type.description)
            .bind(recording_attribute_type.gid)
            .bind(recording_attribute_type.free_text)
            .bind(recording_attribute_type.id)
            .bind(recording_attribute_type.parent)
            .bind(recording_attribute_type.name)
            .bind(recording_attribute_type.comment)
            .bind(recording_attribute_type.child_order)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_attribute_type: RecordingAttributeType) -> Result<RecordingAttributeType> {
        query_as::<_, RecordingAttributeType>(r#"UPDATE "recording_attribute_type" SET "name" = $2, "gid" = $8, "free_text" = $4, "child_order" = $6, "comment" = $3, "parent" = $5, "description" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(recording_attribute_type.child_order)
            .bind(recording_attribute_type.description)
            .bind(recording_attribute_type.name)
            .bind(recording_attribute_type.id)
            .bind(recording_attribute_type.free_text)
            .bind(recording_attribute_type.parent)
            .bind(recording_attribute_type.comment)
            .bind(recording_attribute_type.gid)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_attribute_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
