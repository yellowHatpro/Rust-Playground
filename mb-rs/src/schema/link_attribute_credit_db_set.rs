#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkAttributeCredit;

pub struct LinkAttributeCreditSet;

impl LinkAttributeCreditSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_link_and_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_link_and_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, link_list: Vec<i32>, attribute_type_list: Vec<i32>) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = ANY($1) AND "attribute_type" = ANY($2)"#)
            .bind(link_list)
            .bind(attribute_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_link_and_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_link_creditable_attribute_type_attribute_type_where_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, link_creditable_attribute_type_attribute_type: i32) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE attribute_type = $1"#)
            .bind(link_creditable_attribute_type_attribute_type)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_credit: LinkAttributeCredit) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"INSERT INTO "link_attribute_credit" ("link", "attribute_type", "credited_as") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(link_attribute_credit.link)
            .bind(link_attribute_credit.credited_as)
            .bind(link_attribute_credit.attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_credit: LinkAttributeCredit) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"UPDATE "link_attribute_credit" SET "credited_as" = $3 WHERE "attribute_type" = 2 AND "link" = 1 RETURNING *;"#)
            .bind(link_attribute_credit.link)
            .bind(link_attribute_credit.credited_as)
            .bind(link_attribute_credit.attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_attribute_credit" WHERE "link" = 1 AND "attribute_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
