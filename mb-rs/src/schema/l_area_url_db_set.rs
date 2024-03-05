#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LAreaUrl;

pub struct LAreaUrlSet;

impl LAreaUrlSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LAreaUrl> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE entity0 = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_url_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, url_id: i32) -> Result<Vec<LAreaUrl>> {
        query_as::<_, LAreaUrl>(r#"SELECT * FROM "musicbrainz"."l_area_url" WHERE entity1 = $1"#)
            .bind(url_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_url: LAreaUrl) -> Result<LAreaUrl> {
        query_as::<_, LAreaUrl>(r#"INSERT INTO "l_area_url" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_area_url.link)
            .bind(l_area_url.entity1)
            .bind(l_area_url.last_updated)
            .bind(l_area_url.entity0_credit)
            .bind(l_area_url.link_order)
            .bind(l_area_url.entity1_credit)
            .bind(l_area_url.entity0)
            .bind(l_area_url.id)
            .bind(l_area_url.edits_pending)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_url: LAreaUrl) -> Result<LAreaUrl> {
        query_as::<_, LAreaUrl>(r#"UPDATE "l_area_url" SET "last_updated" = $6, "entity1_credit" = $9, "link" = $2, "entity0_credit" = $8, "edits_pending" = $5, "entity1" = $4, "link_order" = $7, "entity0" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_area_url.entity1)
            .bind(l_area_url.entity0)
            .bind(l_area_url.link_order)
            .bind(l_area_url.last_updated)
            .bind(l_area_url.entity1_credit)
            .bind(l_area_url.edits_pending)
            .bind(l_area_url.link)
            .bind(l_area_url.entity0_credit)
            .bind(l_area_url.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_area_url" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
