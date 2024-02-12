#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LAreaLabel;

pub struct LAreaLabelSet;

impl LAreaLabelSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_id<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE entity0 = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LAreaLabel>> {
        query_as::<_, LAreaLabel>(r#"SELECT * FROM "musicbrainz"."l_area_label" WHERE entity1 = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_label: LAreaLabel) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"INSERT INTO "l_area_label" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_area_label.id)
            .bind(l_area_label.link)
            .bind(l_area_label.entity0)
            .bind(l_area_label.entity1)
            .bind(l_area_label.edits_pending)
            .bind(l_area_label.last_updated)
            .bind(l_area_label.link_order)
            .bind(l_area_label.entity0_credit)
            .bind(l_area_label.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_label: LAreaLabel) -> Result<LAreaLabel> {
        query_as::<_, LAreaLabel>(r#"UPDATE "l_area_label" SET "link" = $2, "entity0" = $3, "entity1" = $4, "edits_pending" = $5, "last_updated" = $6, "link_order" = $7, "entity0_credit" = $8, "entity1_credit" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_area_label.id)
            .bind(l_area_label.link)
            .bind(l_area_label.entity0)
            .bind(l_area_label.entity1)
            .bind(l_area_label.edits_pending)
            .bind(l_area_label.last_updated)
            .bind(l_area_label.link_order)
            .bind(l_area_label.entity0_credit)
            .bind(l_area_label.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_area_label" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
