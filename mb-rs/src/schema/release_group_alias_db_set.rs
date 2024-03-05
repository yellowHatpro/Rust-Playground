#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAlias;

pub struct ReleaseGroupAliasSet;

impl ReleaseGroupAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, release_group_alias_type_id: i32) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE type = $1"#)
            .bind(release_group_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias: ReleaseGroupAlias) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"INSERT INTO "release_group_alias" ("id", "release_group", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(release_group_alias.end_date_day)
            .bind(release_group_alias.begin_date_month)
            .bind(release_group_alias.primary_for_locale)
            .bind(release_group_alias.name)
            .bind(release_group_alias.end_date_year)
            .bind(release_group_alias.end_date_month)
            .bind(release_group_alias.release_group)
            .bind(release_group_alias.begin_date_day)
            .bind(release_group_alias.edits_pending)
            .bind(release_group_alias.begin_date_year)
            .bind(release_group_alias.last_updated)
            .bind(release_group_alias.locale)
            .bind(release_group_alias.id)
            .bind(release_group_alias.sort_name)
            .bind(release_group_alias.ended)
            .bind(release_group_alias.Type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias: ReleaseGroupAlias) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"UPDATE "release_group_alias" SET "begin_date_day" = $11, "begin_date_year" = $9, "end_date_day" = $14, "last_updated" = $6, "end_date_year" = $12, "sort_name" = $8, "name" = $3, "end_date_month" = $13, "locale" = $4, "ended" = $16, "primary_for_locale" = $15, "type" = $7, "begin_date_month" = $10, "release_group" = $2, "edits_pending" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_alias.primary_for_locale)
            .bind(release_group_alias.ended)
            .bind(release_group_alias.end_date_year)
            .bind(release_group_alias.end_date_month)
            .bind(release_group_alias.locale)
            .bind(release_group_alias.begin_date_month)
            .bind(release_group_alias.Type)
            .bind(release_group_alias.begin_date_year)
            .bind(release_group_alias.begin_date_day)
            .bind(release_group_alias.id)
            .bind(release_group_alias.name)
            .bind(release_group_alias.end_date_day)
            .bind(release_group_alias.edits_pending)
            .bind(release_group_alias.last_updated)
            .bind(release_group_alias.release_group)
            .bind(release_group_alias.sort_name)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
