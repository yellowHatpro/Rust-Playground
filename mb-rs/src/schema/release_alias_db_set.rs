#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseAlias;

pub struct ReleaseAliasSet;

impl ReleaseAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseAlias>> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseAlias> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseAlias>> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseAlias>> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseAlias>> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, release_alias_type_id: i32) -> Result<Vec<ReleaseAlias>> {
        query_as::<_, ReleaseAlias>(r#"SELECT * FROM "musicbrainz"."release_alias" WHERE type = $1"#)
            .bind(release_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_alias: ReleaseAlias) -> Result<ReleaseAlias> {
        query_as::<_, ReleaseAlias>(r#"INSERT INTO "release_alias" ("id", "release", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(release_alias.primary_for_locale)
            .bind(release_alias.sort_name)
            .bind(release_alias.begin_date_year)
            .bind(release_alias.edits_pending)
            .bind(release_alias.end_date_year)
            .bind(release_alias.id)
            .bind(release_alias.release)
            .bind(release_alias.ended)
            .bind(release_alias.name)
            .bind(release_alias.Type)
            .bind(release_alias.begin_date_month)
            .bind(release_alias.end_date_day)
            .bind(release_alias.end_date_month)
            .bind(release_alias.last_updated)
            .bind(release_alias.begin_date_day)
            .bind(release_alias.locale)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_alias: ReleaseAlias) -> Result<ReleaseAlias> {
        query_as::<_, ReleaseAlias>(r#"UPDATE "release_alias" SET "locale" = $4, "name" = $3, "end_date_year" = $12, "end_date_day" = $14, "end_date_month" = $13, "last_updated" = $6, "begin_date_day" = $11, "begin_date_year" = $9, "ended" = $16, "begin_date_month" = $10, "type" = $7, "edits_pending" = $5, "release" = $2, "primary_for_locale" = $15, "sort_name" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_alias.locale)
            .bind(release_alias.begin_date_month)
            .bind(release_alias.primary_for_locale)
            .bind(release_alias.begin_date_day)
            .bind(release_alias.begin_date_year)
            .bind(release_alias.name)
            .bind(release_alias.end_date_day)
            .bind(release_alias.id)
            .bind(release_alias.end_date_month)
            .bind(release_alias.sort_name)
            .bind(release_alias.end_date_year)
            .bind(release_alias.Type)
            .bind(release_alias.last_updated)
            .bind(release_alias.edits_pending)
            .bind(release_alias.release)
            .bind(release_alias.ended)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
