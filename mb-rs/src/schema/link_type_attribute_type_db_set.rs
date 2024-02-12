#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkTypeAttributeType;

pub struct LinkTypeAttributeTypeSet;

impl LinkTypeAttributeTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_link_type_and_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: i32, attribute_type: i32) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "link_type" = $1 AND "attribute_type" = $2"#)
            .bind(link_type)
            .bind(attribute_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_link_type_and_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, link_type_list: Vec<i32>, attribute_type_list: Vec<i32>) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "link_type" = ANY($1) AND "attribute_type" = ANY($2)"#)
            .bind(link_type_list)
            .bind(attribute_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_link_type_and_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: i32, attribute_type: i32) -> Result<Option<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "link_type" = $1 AND "attribute_type" = $2"#)
            .bind(link_type)
            .bind(attribute_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_type_id<'e, E: PgExecutor<'e>>(executor: E, link_type_id: i32) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE link_type = $1"#)
            .bind(link_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_link_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, link_attribute_type_id: i32) -> Result<Vec<LinkTypeAttributeType>> {
        query_as::<_, LinkTypeAttributeType>(r#"SELECT * FROM "musicbrainz"."link_type_attribute_type" WHERE attribute_type = $1"#)
            .bind(link_attribute_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_type_attribute_type: LinkTypeAttributeType) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"INSERT INTO "link_type_attribute_type" ("link_type", "attribute_type", "min", "max", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(link_type_attribute_type.link_type)
            .bind(link_type_attribute_type.attribute_type)
            .bind(link_type_attribute_type.min)
            .bind(link_type_attribute_type.max)
            .bind(link_type_attribute_type.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_type_attribute_type: LinkTypeAttributeType) -> Result<LinkTypeAttributeType> {
        query_as::<_, LinkTypeAttributeType>(r#"UPDATE "link_type_attribute_type" SET "min" = $3, "max" = $4, "last_updated" = $5 WHERE "link_type" = 1 AND "attribute_type" = 2 RETURNING *;"#)
            .bind(link_type_attribute_type.link_type)
            .bind(link_type_attribute_type.attribute_type)
            .bind(link_type_attribute_type.min)
            .bind(link_type_attribute_type.max)
            .bind(link_type_attribute_type.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_type_attribute_type" WHERE "link_type" = 1 AND "attribute_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
