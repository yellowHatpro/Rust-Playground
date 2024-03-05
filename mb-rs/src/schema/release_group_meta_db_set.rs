#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupMeta;

pub struct ReleaseGroupMetaSet;

impl ReleaseGroupMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupMeta>> {
        query_as::<_, ReleaseGroupMeta>(r#"SELECT * FROM "musicbrainz"."release_group_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupMeta> {
        query_as::<_, ReleaseGroupMeta>(r#"SELECT * FROM "musicbrainz"."release_group_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupMeta>> {
        query_as::<_, ReleaseGroupMeta>(r#"SELECT * FROM "musicbrainz"."release_group_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupMeta>> {
        query_as::<_, ReleaseGroupMeta>(r#"SELECT * FROM "musicbrainz"."release_group_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupMeta>> {
        query_as::<_, ReleaseGroupMeta>(r#"SELECT * FROM "musicbrainz"."release_group_meta" WHERE id = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_meta: ReleaseGroupMeta) -> Result<ReleaseGroupMeta> {
        query_as::<_, ReleaseGroupMeta>(r#"INSERT INTO "release_group_meta" ("id", "release_count", "first_release_date_year", "first_release_date_month", "first_release_date_day", "rating", "rating_count") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(release_group_meta.first_release_date_month)
            .bind(release_group_meta.first_release_date_day)
            .bind(release_group_meta.rating)
            .bind(release_group_meta.rating_count)
            .bind(release_group_meta.first_release_date_year)
            .bind(release_group_meta.id)
            .bind(release_group_meta.release_count)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_meta: ReleaseGroupMeta) -> Result<ReleaseGroupMeta> {
        query_as::<_, ReleaseGroupMeta>(r#"UPDATE "release_group_meta" SET "first_release_date_month" = $4, "rating_count" = $7, "release_count" = $2, "first_release_date_day" = $5, "rating" = $6, "first_release_date_year" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_meta.release_count)
            .bind(release_group_meta.first_release_date_year)
            .bind(release_group_meta.first_release_date_day)
            .bind(release_group_meta.rating)
            .bind(release_group_meta.rating_count)
            .bind(release_group_meta.first_release_date_month)
            .bind(release_group_meta.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
