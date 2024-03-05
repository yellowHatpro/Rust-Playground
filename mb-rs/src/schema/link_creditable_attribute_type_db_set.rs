#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkCreditableAttributeType;

pub struct LinkCreditableAttributeTypeSet;

impl LinkCreditableAttributeTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkCreditableAttributeType>> {
        query_as::<_, LinkCreditableAttributeType>(r#"SELECT * FROM "musicbrainz"."link_creditable_attribute_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, attribute_type: i32) -> Result<LinkCreditableAttributeType> {
        query_as::<_, LinkCreditableAttributeType>(r#"SELECT * FROM "musicbrainz"."link_creditable_attribute_type" WHERE "attribute_type" = $1"#)
            .bind(attribute_type)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, attribute_type_list: Vec<i32>) -> Result<Vec<LinkCreditableAttributeType>> {
        query_as::<_, LinkCreditableAttributeType>(r#"SELECT * FROM "musicbrainz"."link_creditable_attribute_type" WHERE "attribute_type" = ANY($1)"#)
            .bind(attribute_type_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, attribute_type: i32) -> Result<Option<LinkCreditableAttributeType>> {
        query_as::<_, LinkCreditableAttributeType>(r#"SELECT * FROM "musicbrainz"."link_creditable_attribute_type" WHERE "attribute_type" = $1"#)
            .bind(attribute_type)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_attribute_type_id_where_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, link_attribute_type_id: i32) -> Result<Vec<LinkCreditableAttributeType>> {
        query_as::<_, LinkCreditableAttributeType>(r#"SELECT * FROM "musicbrainz"."link_creditable_attribute_type" WHERE attribute_type = $1"#)
            .bind(link_attribute_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_creditable_attribute_type: LinkCreditableAttributeType) -> Result<LinkCreditableAttributeType> {
        query_as::<_, LinkCreditableAttributeType>(r#"INSERT INTO "link_creditable_attribute_type" ("attribute_type") VALUES ($1) RETURNING *;"#)
            .bind(link_creditable_attribute_type.attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_creditable_attribute_type: LinkCreditableAttributeType) -> Result<LinkCreditableAttributeType> {
        query_as::<_, LinkCreditableAttributeType>(r#"UPDATE "link_creditable_attribute_type" SET  WHERE "attribute_type" = 1 RETURNING *;"#)
            .bind(link_creditable_attribute_type.attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_creditable_attribute_type" WHERE "attribute_type" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
