#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventAttribute;

pub struct EventAttributeSet;

impl EventAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EventAttribute> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_id_where_event_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_attribute_type_id_where_event_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, event_attribute_type_id: i32) -> Result<Vec<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE event_attribute_type = $1"#)
            .bind(event_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_attribute_type_allowed_value_id_where_event_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, event_attribute_type_allowed_value_id: i32) -> Result<Vec<EventAttribute>> {
        query_as::<_, EventAttribute>(r#"SELECT * FROM "musicbrainz"."event_attribute" WHERE event_attribute_type_allowed_value = $1"#)
            .bind(event_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_attribute: EventAttribute) -> Result<EventAttribute> {
        query_as::<_, EventAttribute>(r#"INSERT INTO "event_attribute" ("id", "event", "event_attribute_type", "event_attribute_type_allowed_value", "event_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(event_attribute.event_attribute_text)
            .bind(event_attribute.id)
            .bind(event_attribute.event_attribute_type)
            .bind(event_attribute.event_attribute_type_allowed_value)
            .bind(event_attribute.event)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_attribute: EventAttribute) -> Result<EventAttribute> {
        query_as::<_, EventAttribute>(r#"UPDATE "event_attribute" SET "event" = $2, "event_attribute_type" = $3, "event_attribute_type_allowed_value" = $4, "event_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(event_attribute.event_attribute_text)
            .bind(event_attribute.event_attribute_type)
            .bind(event_attribute.event)
            .bind(event_attribute.id)
            .bind(event_attribute.event_attribute_type_allowed_value)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
