#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LReleaseGroupReleaseGroup;

pub struct LReleaseGroupReleaseGroupSet;

impl LReleaseGroupReleaseGroupSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_id<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE entity0 = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<LReleaseGroupReleaseGroup>> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_release_group_release_group" WHERE entity1 = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_release_group_release_group: LReleaseGroupReleaseGroup) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"INSERT INTO "l_release_group_release_group" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_release_group_release_group.id)
            .bind(l_release_group_release_group.link)
            .bind(l_release_group_release_group.entity0)
            .bind(l_release_group_release_group.entity1)
            .bind(l_release_group_release_group.edits_pending)
            .bind(l_release_group_release_group.last_updated)
            .bind(l_release_group_release_group.link_order)
            .bind(l_release_group_release_group.entity0_credit)
            .bind(l_release_group_release_group.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_release_group_release_group: LReleaseGroupReleaseGroup) -> Result<LReleaseGroupReleaseGroup> {
        query_as::<_, LReleaseGroupReleaseGroup>(r#"UPDATE "l_release_group_release_group" SET "link" = $2, "entity0" = $3, "entity1" = $4, "edits_pending" = $5, "last_updated" = $6, "link_order" = $7, "entity0_credit" = $8, "entity1_credit" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_release_group_release_group.id)
            .bind(l_release_group_release_group.link)
            .bind(l_release_group_release_group.entity0)
            .bind(l_release_group_release_group.entity1)
            .bind(l_release_group_release_group.edits_pending)
            .bind(l_release_group_release_group.last_updated)
            .bind(l_release_group_release_group.link_order)
            .bind(l_release_group_release_group.entity0_credit)
            .bind(l_release_group_release_group.entity1_credit)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_release_group_release_group" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
