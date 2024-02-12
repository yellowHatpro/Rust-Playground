#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LEventSeries;

pub struct LEventSeriesSet;

impl LEventSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_id<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE entity0 = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_id<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<LEventSeries>> {
        query_as::<_, LEventSeries>(r#"SELECT * FROM "musicbrainz"."l_event_series" WHERE entity1 = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_event_series: LEventSeries) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"INSERT INTO "l_event_series" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_event_series.id)
            .bind(l_event_series.link)
            .bind(l_event_series.entity0)
            .bind(l_event_series.entity1)
            .bind(l_event_series.edits_pending)
            .bind(l_event_series.last_updated)
            .bind(l_event_series.link_order)
            .bind(l_event_series.entity0_credit)
            .bind(l_event_series.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_event_series: LEventSeries) -> Result<LEventSeries> {
        query_as::<_, LEventSeries>(r#"UPDATE "l_event_series" SET "link" = $2, "entity0" = $3, "entity1" = $4, "edits_pending" = $5, "last_updated" = $6, "link_order" = $7, "entity0_credit" = $8, "entity1_credit" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_event_series.id)
            .bind(l_event_series.link)
            .bind(l_event_series.entity0)
            .bind(l_event_series.entity1)
            .bind(l_event_series.edits_pending)
            .bind(l_event_series.last_updated)
            .bind(l_event_series.link_order)
            .bind(l_event_series.entity0_credit)
            .bind(l_event_series.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_event_series" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
