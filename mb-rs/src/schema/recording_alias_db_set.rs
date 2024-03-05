#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::RecordingAlias;

pub struct RecordingAliasSet;

impl RecordingAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<RecordingAlias>> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<RecordingAlias> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<RecordingAlias>> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<RecordingAlias>> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_recording_id_where_recording_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<RecordingAlias>> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias" WHERE recording = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, recording_alias_type_id: i32) -> Result<Vec<RecordingAlias>> {
        query_as::<_, RecordingAlias>(r#"SELECT * FROM "musicbrainz"."recording_alias" WHERE type = $1"#)
            .bind(recording_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, recording_alias: RecordingAlias) -> Result<RecordingAlias> {
        query_as::<_, RecordingAlias>(r#"INSERT INTO "recording_alias" ("id", "recording", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(recording_alias.end_date_day)
            .bind(recording_alias.edits_pending)
            .bind(recording_alias.begin_date_year)
            .bind(recording_alias.Type)
            .bind(recording_alias.id)
            .bind(recording_alias.begin_date_month)
            .bind(recording_alias.ended)
            .bind(recording_alias.name)
            .bind(recording_alias.last_updated)
            .bind(recording_alias.begin_date_day)
            .bind(recording_alias.locale)
            .bind(recording_alias.end_date_year)
            .bind(recording_alias.recording)
            .bind(recording_alias.primary_for_locale)
            .bind(recording_alias.sort_name)
            .bind(recording_alias.end_date_month)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, recording_alias: RecordingAlias) -> Result<RecordingAlias> {
        query_as::<_, RecordingAlias>(r#"UPDATE "recording_alias" SET "name" = $3, "type" = $7, "sort_name" = $8, "begin_date_month" = $10, "end_date_day" = $14, "last_updated" = $6, "ended" = $16, "begin_date_year" = $9, "edits_pending" = $5, "begin_date_day" = $11, "recording" = $2, "primary_for_locale" = $15, "locale" = $4, "end_date_year" = $12, "end_date_month" = $13 WHERE "id" = 1 RETURNING *;"#)
            .bind(recording_alias.end_date_month)
            .bind(recording_alias.locale)
            .bind(recording_alias.end_date_year)
            .bind(recording_alias.last_updated)
            .bind(recording_alias.sort_name)
            .bind(recording_alias.primary_for_locale)
            .bind(recording_alias.Type)
            .bind(recording_alias.name)
            .bind(recording_alias.id)
            .bind(recording_alias.ended)
            .bind(recording_alias.recording)
            .bind(recording_alias.begin_date_month)
            .bind(recording_alias.end_date_day)
            .bind(recording_alias.edits_pending)
            .bind(recording_alias.begin_date_year)
            .bind(recording_alias.begin_date_day)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."recording_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
