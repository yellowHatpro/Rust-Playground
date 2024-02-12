#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaContainment;

pub struct AreaContainmentSet;

impl AreaContainmentSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_descendant_and_parent<'e, E: PgExecutor<'e>>(&self, executor: E, descendant: i32, parent: i32) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = $1 AND "parent" = $2"#)
            .bind(descendant)
            .bind(parent)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_descendant_and_parent_list<'e, E: PgExecutor<'e>>(&self, executor: E, descendant_list: Vec<i32>, parent_list: Vec<i32>) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = ANY($1) AND "parent" = ANY($2)"#)
            .bind(descendant_list)
            .bind(parent_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_descendant_and_parent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, descendant: i32, parent: i32) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = $1 AND "parent" = $2"#)
            .bind(descendant)
            .bind(parent)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE descendant = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE parent = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_containment: AreaContainment) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"INSERT INTO "area_containment" ("descendant", "parent", "depth") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(area_containment.descendant)
            .bind(area_containment.parent)
            .bind(area_containment.depth)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_containment: AreaContainment) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"UPDATE "area_containment" SET "depth" = $3 WHERE "descendant" = 1 AND "parent" = 2 RETURNING *;"#)
            .bind(area_containment.descendant)
            .bind(area_containment.parent)
            .bind(area_containment.depth)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_containment" WHERE "descendant" = 1 AND "parent" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
