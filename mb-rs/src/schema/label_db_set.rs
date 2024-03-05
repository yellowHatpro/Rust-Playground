#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Label;

pub struct LabelSet;

impl LabelSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Label>> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Label> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Label>> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Label>> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, label_type_id: i32) -> Result<Vec<Label>> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label" WHERE type = $1"#)
            .bind(label_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Label>> {
        query_as::<_, Label>(r#"SELECT * FROM "musicbrainz"."label" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label: Label) -> Result<Label> {
        query_as::<_, Label>(r#"INSERT INTO "label" ("id", "gid", "name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "label_code", "type", "area", "comment", "edits_pending", "last_updated", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(label.id)
            .bind(label.begin_date_year)
            .bind(label.end_date_day)
            .bind(label.comment)
            .bind(label.gid)
            .bind(label.area)
            .bind(label.begin_date_month)
            .bind(label.Type)
            .bind(label.last_updated)
            .bind(label.ended)
            .bind(label.edits_pending)
            .bind(label.end_date_month)
            .bind(label.begin_date_day)
            .bind(label.name)
            .bind(label.end_date_year)
            .bind(label.label_code)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label: Label) -> Result<Label> {
        query_as::<_, Label>(r#"UPDATE "label" SET "end_date_month" = $8, "begin_date_day" = $6, "ended" = $16, "edits_pending" = $14, "last_updated" = $15, "begin_date_month" = $5, "label_code" = $10, "gid" = $2, "type" = $11, "area" = $12, "comment" = $13, "name" = $3, "begin_date_year" = $4, "end_date_year" = $7, "end_date_day" = $9 WHERE "id" = 1 RETURNING *;"#)
            .bind(label.label_code)
            .bind(label.end_date_day)
            .bind(label.area)
            .bind(label.last_updated)
            .bind(label.end_date_year)
            .bind(label.begin_date_day)
            .bind(label.comment)
            .bind(label.ended)
            .bind(label.Type)
            .bind(label.begin_date_month)
            .bind(label.id)
            .bind(label.begin_date_year)
            .bind(label.end_date_month)
            .bind(label.edits_pending)
            .bind(label.name)
            .bind(label.gid)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
