#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelAlias;

pub struct LabelAliasSet;

impl LabelAliasSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_alias_type_id<'e, E: PgExecutor<'e>>(executor: E, label_alias_type_id: i32) -> Result<Vec<LabelAlias>> {
        query_as::<_, LabelAlias>(r#"SELECT * FROM "musicbrainz"."label_alias" WHERE type = $1"#)
            .bind(label_alias_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_alias: LabelAlias) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"INSERT INTO "label_alias" ("id", "label", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(label_alias.id)
            .bind(label_alias.label)
            .bind(label_alias.name)
            .bind(label_alias.locale)
            .bind(label_alias.edits_pending)
            .bind(label_alias.last_updated)
            .bind(label_alias.Type)
            .bind(label_alias.sort_name)
            .bind(label_alias.begin_date_year)
            .bind(label_alias.begin_date_month)
            .bind(label_alias.begin_date_day)
            .bind(label_alias.end_date_year)
            .bind(label_alias.end_date_month)
            .bind(label_alias.end_date_day)
            .bind(label_alias.primary_for_locale)
            .bind(label_alias.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_alias: LabelAlias) -> Result<LabelAlias> {
        query_as::<_, LabelAlias>(r#"UPDATE "label_alias" SET "label" = $2, "name" = $3, "locale" = $4, "edits_pending" = $5, "last_updated" = $6, "type" = $7, "sort_name" = $8, "begin_date_year" = $9, "begin_date_month" = $10, "begin_date_day" = $11, "end_date_year" = $12, "end_date_month" = $13, "end_date_day" = $14, "primary_for_locale" = $15, "ended" = $16 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_alias.id)
            .bind(label_alias.label)
            .bind(label_alias.name)
            .bind(label_alias.locale)
            .bind(label_alias.edits_pending)
            .bind(label_alias.last_updated)
            .bind(label_alias.Type)
            .bind(label_alias.sort_name)
            .bind(label_alias.begin_date_year)
            .bind(label_alias.begin_date_month)
            .bind(label_alias.begin_date_day)
            .bind(label_alias.end_date_year)
            .bind(label_alias.end_date_month)
            .bind(label_alias.end_date_day)
            .bind(label_alias.primary_for_locale)
            .bind(label_alias.ended)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
