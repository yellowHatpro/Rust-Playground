#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Link;

pub struct LinkSet;

impl LinkSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Link>> {
        query_as::<_, Link>(r#"SELECT * FROM "musicbrainz"."link""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Link> {
        query_as::<_, Link>(r#"SELECT * FROM "musicbrainz"."link" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Link>> {
        query_as::<_, Link>(r#"SELECT * FROM "musicbrainz"."link" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Link>> {
        query_as::<_, Link>(r#"SELECT * FROM "musicbrainz"."link" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_type_id_where_link_type_is<'e, E: PgExecutor<'e>>(executor: E, link_type_id: i32) -> Result<Vec<Link>> {
        query_as::<_, Link>(r#"SELECT * FROM "musicbrainz"."link" WHERE link_type = $1"#)
            .bind(link_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link: Link) -> Result<Link> {
        query_as::<_, Link>(r#"INSERT INTO "link" ("id", "link_type", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "attribute_count", "created", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *;"#)
            .bind(link.end_date_day)
            .bind(link.begin_date_year)
            .bind(link.ended)
            .bind(link.end_date_month)
            .bind(link.id)
            .bind(link.attribute_count)
            .bind(link.begin_date_month)
            .bind(link.link_type)
            .bind(link.end_date_year)
            .bind(link.created)
            .bind(link.begin_date_day)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link: Link) -> Result<Link> {
        query_as::<_, Link>(r#"UPDATE "link" SET "link_type" = $2, "end_date_day" = $8, "end_date_month" = $7, "begin_date_year" = $3, "attribute_count" = $9, "created" = $10, "end_date_year" = $6, "ended" = $11, "begin_date_month" = $4, "begin_date_day" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(link.end_date_month)
            .bind(link.begin_date_month)
            .bind(link.id)
            .bind(link.begin_date_day)
            .bind(link.end_date_year)
            .bind(link.end_date_day)
            .bind(link.begin_date_year)
            .bind(link.attribute_count)
            .bind(link.created)
            .bind(link.link_type)
            .bind(link.ended)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
