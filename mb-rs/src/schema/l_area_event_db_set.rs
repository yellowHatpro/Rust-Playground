#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LAreaEvent;

pub struct LAreaEventSet;

impl LAreaEventSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LAreaEvent> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE entity0 = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<LAreaEvent>> {
        query_as::<_, LAreaEvent>(r#"SELECT * FROM "musicbrainz"."l_area_event" WHERE entity1 = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_event: LAreaEvent) -> Result<LAreaEvent> {
        query_as::<_, LAreaEvent>(r#"INSERT INTO "l_area_event" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_area_event.entity1_credit)
            .bind(l_area_event.entity0_credit)
            .bind(l_area_event.link)
            .bind(l_area_event.edits_pending)
            .bind(l_area_event.last_updated)
            .bind(l_area_event.entity1)
            .bind(l_area_event.entity0)
            .bind(l_area_event.link_order)
            .bind(l_area_event.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_event: LAreaEvent) -> Result<LAreaEvent> {
        query_as::<_, LAreaEvent>(r#"UPDATE "l_area_event" SET "entity1_credit" = $9, "last_updated" = $6, "edits_pending" = $5, "entity0_credit" = $8, "entity0" = $3, "link" = $2, "link_order" = $7, "entity1" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_area_event.link)
            .bind(l_area_event.entity1)
            .bind(l_area_event.entity0_credit)
            .bind(l_area_event.edits_pending)
            .bind(l_area_event.id)
            .bind(l_area_event.last_updated)
            .bind(l_area_event.entity1_credit)
            .bind(l_area_event.link_order)
            .bind(l_area_event.entity0)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_area_event" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
