#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Area;

pub struct AreaSet;

impl AreaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Area>> {
        query_as::<_, Area>(r#"SELECT * FROM "musicbrainz"."area""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Area> {
        query_as::<_, Area>(r#"SELECT * FROM "musicbrainz"."area" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Area>> {
        query_as::<_, Area>(r#"SELECT * FROM "musicbrainz"."area" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Area>> {
        query_as::<_, Area>(r#"SELECT * FROM "musicbrainz"."area" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, area_type_id: i32) -> Result<Vec<Area>> {
        query_as::<_, Area>(r#"SELECT * FROM "musicbrainz"."area" WHERE type = $1"#)
            .bind(area_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area: Area) -> Result<Area> {
        query_as::<_, Area>(r#"INSERT INTO "area" ("id", "gid", "name", "type", "edits_pending", "last_updated", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "ended", "comment") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING *;"#)
            .bind(area.id)
            .bind(area.last_updated)
            .bind(area.begin_date_month)
            .bind(area.name)
            .bind(area.gid)
            .bind(area.Type)
            .bind(area.begin_date_day)
            .bind(area.end_date_month)
            .bind(area.edits_pending)
            .bind(area.comment)
            .bind(area.end_date_day)
            .bind(area.begin_date_year)
            .bind(area.end_date_year)
            .bind(area.ended)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area: Area) -> Result<Area> {
        query_as::<_, Area>(r#"UPDATE "area" SET "name" = $3, "begin_date_day" = $9, "end_date_year" = $10, "begin_date_month" = $8, "begin_date_year" = $7, "type" = $4, "end_date_day" = $12, "ended" = $13, "gid" = $2, "edits_pending" = $5, "end_date_month" = $11, "comment" = $14, "last_updated" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(area.begin_date_month)
            .bind(area.begin_date_day)
            .bind(area.end_date_year)
            .bind(area.comment)
            .bind(area.name)
            .bind(area.last_updated)
            .bind(area.gid)
            .bind(area.begin_date_year)
            .bind(area.id)
            .bind(area.Type)
            .bind(area.end_date_month)
            .bind(area.ended)
            .bind(area.end_date_day)
            .bind(area.edits_pending)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
