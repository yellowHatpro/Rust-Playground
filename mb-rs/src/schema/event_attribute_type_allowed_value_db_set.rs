#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventAttributeTypeAllowedValue;

pub struct EventAttributeTypeAllowedValueSet;

impl EventAttributeTypeAllowedValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventAttributeTypeAllowedValue>> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EventAttributeTypeAllowedValue> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EventAttributeTypeAllowedValue>> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EventAttributeTypeAllowedValue>> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_attribute_type_id_where_event_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, event_attribute_type_id: i32) -> Result<Vec<EventAttributeTypeAllowedValue>> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE event_attribute_type = $1"#)
            .bind(event_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_attribute_type_allowed_value_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, event_attribute_type_allowed_value_id: i32) -> Result<Vec<EventAttributeTypeAllowedValue>> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(event_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_attribute_type_allowed_value: EventAttributeTypeAllowedValue) -> Result<EventAttributeTypeAllowedValue> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"INSERT INTO "event_attribute_type_allowed_value" ("id", "event_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(event_attribute_type_allowed_value.value)
            .bind(event_attribute_type_allowed_value.gid)
            .bind(event_attribute_type_allowed_value.child_order)
            .bind(event_attribute_type_allowed_value.event_attribute_type)
            .bind(event_attribute_type_allowed_value.id)
            .bind(event_attribute_type_allowed_value.description)
            .bind(event_attribute_type_allowed_value.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_attribute_type_allowed_value: EventAttributeTypeAllowedValue) -> Result<EventAttributeTypeAllowedValue> {
        query_as::<_, EventAttributeTypeAllowedValue>(r#"UPDATE "event_attribute_type_allowed_value" SET "description" = $6, "event_attribute_type" = $2, "value" = $3, "gid" = $7, "child_order" = $5, "parent" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(event_attribute_type_allowed_value.value)
            .bind(event_attribute_type_allowed_value.gid)
            .bind(event_attribute_type_allowed_value.id)
            .bind(event_attribute_type_allowed_value.parent)
            .bind(event_attribute_type_allowed_value.event_attribute_type)
            .bind(event_attribute_type_allowed_value.child_order)
            .bind(event_attribute_type_allowed_value.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
