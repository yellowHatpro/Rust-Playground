#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkAttributeTextValue;

pub struct LinkAttributeTextValueSet;

impl LinkAttributeTextValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkAttributeTextValue>> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_link_and_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<LinkAttributeTextValue> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_link_and_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, link_list: Vec<i32>, attribute_type_list: Vec<i32>) -> Result<Vec<LinkAttributeTextValue>> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value" WHERE "link" = ANY($1) AND "attribute_type" = ANY($2)"#)
            .bind(link_list)
            .bind(attribute_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_link_and_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<Option<LinkAttributeTextValue>> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LinkAttributeTextValue>> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_link_text_attribute_type_attribute_type_where_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, link_text_attribute_type_attribute_type: i32) -> Result<Vec<LinkAttributeTextValue>> {
        query_as::<_, LinkAttributeTextValue>(r#"SELECT * FROM "musicbrainz"."link_attribute_text_value" WHERE attribute_type = $1"#)
            .bind(link_text_attribute_type_attribute_type)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_text_value: LinkAttributeTextValue) -> Result<LinkAttributeTextValue> {
        query_as::<_, LinkAttributeTextValue>(r#"INSERT INTO "link_attribute_text_value" ("link", "attribute_type", "text_value") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(link_attribute_text_value.attribute_type)
            .bind(link_attribute_text_value.text_value)
            .bind(link_attribute_text_value.link)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_text_value: LinkAttributeTextValue) -> Result<LinkAttributeTextValue> {
        query_as::<_, LinkAttributeTextValue>(r#"UPDATE "link_attribute_text_value" SET "text_value" = $3 WHERE "link" = 1 AND "attribute_type" = 2 RETURNING *;"#)
            .bind(link_attribute_text_value.link)
            .bind(link_attribute_text_value.attribute_type)
            .bind(link_attribute_text_value.text_value)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_attribute_text_value" WHERE "link" = 1 AND "attribute_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
