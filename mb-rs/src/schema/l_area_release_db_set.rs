#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LAreaRelease;

pub struct LAreaReleaseSet;

impl LAreaReleaseSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LAreaRelease> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE entity0 = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<LAreaRelease>> {
        query_as::<_, LAreaRelease>(r#"SELECT * FROM "musicbrainz"."l_area_release" WHERE entity1 = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_release: LAreaRelease) -> Result<LAreaRelease> {
        query_as::<_, LAreaRelease>(r#"INSERT INTO "l_area_release" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_area_release.link_order)
            .bind(l_area_release.entity1)
            .bind(l_area_release.entity0)
            .bind(l_area_release.edits_pending)
            .bind(l_area_release.id)
            .bind(l_area_release.entity0_credit)
            .bind(l_area_release.last_updated)
            .bind(l_area_release.entity1_credit)
            .bind(l_area_release.link)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_area_release: LAreaRelease) -> Result<LAreaRelease> {
        query_as::<_, LAreaRelease>(r#"UPDATE "l_area_release" SET "entity1" = $4, "entity0_credit" = $8, "link" = $2, "edits_pending" = $5, "link_order" = $7, "entity0" = $3, "entity1_credit" = $9, "last_updated" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_area_release.entity0_credit)
            .bind(l_area_release.link)
            .bind(l_area_release.edits_pending)
            .bind(l_area_release.entity1_credit)
            .bind(l_area_release.entity1)
            .bind(l_area_release.link_order)
            .bind(l_area_release.entity0)
            .bind(l_area_release.id)
            .bind(l_area_release.last_updated)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_area_release" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
