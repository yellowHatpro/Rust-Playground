#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupSecondaryTypeJoin;

pub struct ReleaseGroupSecondaryTypeJoinSet;

impl ReleaseGroupSecondaryTypeJoinSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupSecondaryTypeJoin>> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release_group_and_secondary_type<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, secondary_type: i32) -> Result<ReleaseGroupSecondaryTypeJoin> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join" WHERE "release_group" = $1 AND "secondary_type" = $2"#)
            .bind(release_group)
            .bind(secondary_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_group_and_secondary_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_list: Vec<i32>, secondary_type_list: Vec<i32>) -> Result<Vec<ReleaseGroupSecondaryTypeJoin>> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join" WHERE "release_group" = ANY($1) AND "secondary_type" = ANY($2)"#)
            .bind(release_group_list)
            .bind(secondary_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_group_and_secondary_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, secondary_type: i32) -> Result<Option<ReleaseGroupSecondaryTypeJoin>> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join" WHERE "release_group" = $1 AND "secondary_type" = $2"#)
            .bind(release_group)
            .bind(secondary_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupSecondaryTypeJoin>> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_secondary_type_id_where_secondary_type_is<'e, E: PgExecutor<'e>>(executor: E, release_group_secondary_type_id: i32) -> Result<Vec<ReleaseGroupSecondaryTypeJoin>> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"SELECT * FROM "musicbrainz"."release_group_secondary_type_join" WHERE secondary_type = $1"#)
            .bind(release_group_secondary_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_secondary_type_join: ReleaseGroupSecondaryTypeJoin) -> Result<ReleaseGroupSecondaryTypeJoin> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"INSERT INTO "release_group_secondary_type_join" ("release_group", "secondary_type", "created") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(release_group_secondary_type_join.release_group)
            .bind(release_group_secondary_type_join.secondary_type)
            .bind(release_group_secondary_type_join.created)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_secondary_type_join: ReleaseGroupSecondaryTypeJoin) -> Result<ReleaseGroupSecondaryTypeJoin> {
        query_as::<_, ReleaseGroupSecondaryTypeJoin>(r#"UPDATE "release_group_secondary_type_join" SET "created" = $3 WHERE "release_group" = 1 AND "secondary_type" = 2 RETURNING *;"#)
            .bind(release_group_secondary_type_join.secondary_type)
            .bind(release_group_secondary_type_join.created)
            .bind(release_group_secondary_type_join.release_group)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_secondary_type_join" WHERE "release_group" = 1 AND "secondary_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
