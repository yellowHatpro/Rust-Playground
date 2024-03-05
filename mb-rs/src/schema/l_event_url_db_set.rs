#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LEventUrl;

pub struct LEventUrlSet;

impl LEventUrlSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LEventUrl> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE entity0 = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_url_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, url_id: i32) -> Result<Vec<LEventUrl>> {
        query_as::<_, LEventUrl>(r#"SELECT * FROM "musicbrainz"."l_event_url" WHERE entity1 = $1"#)
            .bind(url_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_event_url: LEventUrl) -> Result<LEventUrl> {
        query_as::<_, LEventUrl>(r#"INSERT INTO "l_event_url" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_event_url.edits_pending)
            .bind(l_event_url.link_order)
            .bind(l_event_url.entity1)
            .bind(l_event_url.last_updated)
            .bind(l_event_url.link)
            .bind(l_event_url.entity0_credit)
            .bind(l_event_url.id)
            .bind(l_event_url.entity1_credit)
            .bind(l_event_url.entity0)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_event_url: LEventUrl) -> Result<LEventUrl> {
        query_as::<_, LEventUrl>(r#"UPDATE "l_event_url" SET "entity1" = $4, "entity1_credit" = $9, "last_updated" = $6, "link_order" = $7, "edits_pending" = $5, "entity0" = $3, "entity0_credit" = $8, "link" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_event_url.entity1_credit)
            .bind(l_event_url.link)
            .bind(l_event_url.entity0)
            .bind(l_event_url.entity1)
            .bind(l_event_url.entity0_credit)
            .bind(l_event_url.edits_pending)
            .bind(l_event_url.link_order)
            .bind(l_event_url.id)
            .bind(l_event_url.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_event_url" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
