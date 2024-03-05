#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::OrderableLinkType;

pub struct OrderableLinkTypeSet;

impl OrderableLinkTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<OrderableLinkType>> {
        query_as::<_, OrderableLinkType>(r#"SELECT * FROM "musicbrainz"."orderable_link_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_link_type<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: i32) -> Result<OrderableLinkType> {
        query_as::<_, OrderableLinkType>(r#"SELECT * FROM "musicbrainz"."orderable_link_type" WHERE "link_type" = $1"#)
            .bind(link_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_link_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, link_type_list: Vec<i32>) -> Result<Vec<OrderableLinkType>> {
        query_as::<_, OrderableLinkType>(r#"SELECT * FROM "musicbrainz"."orderable_link_type" WHERE "link_type" = ANY($1)"#)
            .bind(link_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_link_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: i32) -> Result<Option<OrderableLinkType>> {
        query_as::<_, OrderableLinkType>(r#"SELECT * FROM "musicbrainz"."orderable_link_type" WHERE "link_type" = $1"#)
            .bind(link_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_type_id_where_link_type_is<'e, E: PgExecutor<'e>>(executor: E, link_type_id: i32) -> Result<Vec<OrderableLinkType>> {
        query_as::<_, OrderableLinkType>(r#"SELECT * FROM "musicbrainz"."orderable_link_type" WHERE link_type = $1"#)
            .bind(link_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, orderable_link_type: OrderableLinkType) -> Result<OrderableLinkType> {
        query_as::<_, OrderableLinkType>(r#"INSERT INTO "orderable_link_type" ("link_type", "direction") VALUES ($1, $2) RETURNING *;"#)
            .bind(orderable_link_type.link_type)
            .bind(orderable_link_type.direction)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, orderable_link_type: OrderableLinkType) -> Result<OrderableLinkType> {
        query_as::<_, OrderableLinkType>(r#"UPDATE "orderable_link_type" SET "direction" = $2 WHERE "link_type" = 1 RETURNING *;"#)
            .bind(orderable_link_type.link_type)
            .bind(orderable_link_type.direction)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."orderable_link_type" WHERE "link_type" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
