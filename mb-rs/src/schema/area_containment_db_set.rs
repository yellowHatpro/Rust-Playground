#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaContainment;

pub struct AreaContainmentSet;

impl AreaContainmentSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_descendant_and_parent<'e, E: PgExecutor<'e>>(&self, executor: E, descendant: i32, parent: i32) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = $1 AND "parent" = $2"#)
            .bind(descendant)
            .bind(parent)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_descendant_and_parent_list<'e, E: PgExecutor<'e>>(&self, executor: E, descendant_list: Vec<i32>, parent_list: Vec<i32>) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = ANY($1) AND "parent" = ANY($2)"#)
            .bind(descendant_list)
            .bind(parent_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_descendant_and_parent_optional<'e, E: PgExecutor<'e>>(&self, executor: E, descendant: i32, parent: i32) -> Result<Option<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE "descendant" = $1 AND "parent" = $2"#)
            .bind(descendant)
            .bind(parent)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_id_where_descendant_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE descendant = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaContainment>> {
        query_as::<_, AreaContainment>(r#"SELECT * FROM "musicbrainz"."area_containment" WHERE parent = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_containment: AreaContainment) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"INSERT INTO "area_containment" ("descendant", "parent", "depth") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(area_containment.descendant)
            .bind(area_containment.parent)
            .bind(area_containment.depth)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_containment: AreaContainment) -> Result<AreaContainment> {
        query_as::<_, AreaContainment>(r#"UPDATE "area_containment" SET "depth" = $3 WHERE "parent" = 2 AND "descendant" = 1 RETURNING *;"#)
            .bind(area_containment.depth)
            .bind(area_containment.descendant)
            .bind(area_containment.parent)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_containment" WHERE "parent" = 2 AND "descendant" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
