#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAlias;

pub struct ReleaseGroupAliasSet;

impl ReleaseGroupAliasSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_alias_type_id<'e, E: PgExecutor<'e>>(executor: E, release_group_alias_type_id: i32) -> Result<Vec<ReleaseGroupAlias>> {
        query_as::<_, ReleaseGroupAlias>(r#"SELECT * FROM "musicbrainz"."release_group_alias" WHERE type = $1"#)
            .bind(release_group_alias_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias: ReleaseGroupAlias) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"INSERT INTO "release_group_alias" ("id", "release_group", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(release_group_alias.id)
            .bind(release_group_alias.release_group)
            .bind(release_group_alias.name)
            .bind(release_group_alias.locale)
            .bind(release_group_alias.edits_pending)
            .bind(release_group_alias.last_updated)
            .bind(release_group_alias.Type)
            .bind(release_group_alias.sort_name)
            .bind(release_group_alias.begin_date_year)
            .bind(release_group_alias.begin_date_month)
            .bind(release_group_alias.begin_date_day)
            .bind(release_group_alias.end_date_year)
            .bind(release_group_alias.end_date_month)
            .bind(release_group_alias.end_date_day)
            .bind(release_group_alias.primary_for_locale)
            .bind(release_group_alias.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_alias: ReleaseGroupAlias) -> Result<ReleaseGroupAlias> {
        query_as::<_, ReleaseGroupAlias>(r#"UPDATE "release_group_alias" SET "release_group" = $2, "name" = $3, "locale" = $4, "edits_pending" = $5, "last_updated" = $6, "type" = $7, "sort_name" = $8, "begin_date_year" = $9, "begin_date_month" = $10, "begin_date_day" = $11, "end_date_year" = $12, "end_date_month" = $13, "end_date_day" = $14, "primary_for_locale" = $15, "ended" = $16 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_alias.id)
            .bind(release_group_alias.release_group)
            .bind(release_group_alias.name)
            .bind(release_group_alias.locale)
            .bind(release_group_alias.edits_pending)
            .bind(release_group_alias.last_updated)
            .bind(release_group_alias.Type)
            .bind(release_group_alias.sort_name)
            .bind(release_group_alias.begin_date_year)
            .bind(release_group_alias.begin_date_month)
            .bind(release_group_alias.begin_date_day)
            .bind(release_group_alias.end_date_year)
            .bind(release_group_alias.end_date_month)
            .bind(release_group_alias.end_date_day)
            .bind(release_group_alias.primary_for_locale)
            .bind(release_group_alias.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
