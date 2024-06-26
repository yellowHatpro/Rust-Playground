#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EventType;

pub struct EventTypeSet;

impl EventTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EventType>> {
        query_as::<_, EventType>(r#"SELECT * FROM "musicbrainz"."event_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EventType> {
        query_as::<_, EventType>(r#"SELECT * FROM "musicbrainz"."event_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EventType>> {
        query_as::<_, EventType>(r#"SELECT * FROM "musicbrainz"."event_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EventType>> {
        query_as::<_, EventType>(r#"SELECT * FROM "musicbrainz"."event_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_event_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, event_type_id: i32) -> Result<Vec<EventType>> {
        query_as::<_, EventType>(r#"SELECT * FROM "musicbrainz"."event_type" WHERE parent = $1"#)
            .bind(event_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, event_type: EventType) -> Result<EventType> {
        query_as::<_, EventType>(r#"INSERT INTO "event_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(event_type.child_order)
            .bind(event_type.description)
            .bind(event_type.name)
            .bind(event_type.id)
            .bind(event_type.gid)
            .bind(event_type.parent)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, event_type: EventType) -> Result<EventType> {
        query_as::<_, EventType>(r#"UPDATE "event_type" SET "name" = $2, "description" = $5, "gid" = $6, "child_order" = $4, "parent" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(event_type.gid)
            .bind(event_type.description)
            .bind(event_type.parent)
            .bind(event_type.child_order)
            .bind(event_type.name)
            .bind(event_type.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."event_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
