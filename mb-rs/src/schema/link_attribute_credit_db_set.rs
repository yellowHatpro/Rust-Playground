#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkAttributeCredit;

pub struct LinkAttributeCreditSet;

impl LinkAttributeCreditSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_link_and_attribute_type<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_link_and_attribute_type_list<'e, E: PgExecutor<'e>>(&self, executor: E, link_list: Vec<i32>, attribute_type_list: Vec<i32>) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = ANY($1) AND "attribute_type" = ANY($2)"#)
            .bind(link_list)
            .bind(attribute_type_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_link_and_attribute_type_optional<'e, E: PgExecutor<'e>>(&self, executor: E, link: i32, attribute_type: i32) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "link" = $1 AND "attribute_type" = $2"#)
            .bind(link)
            .bind(attribute_type)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_id<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_link_creditable_attribute_type_attribute_type<'e, E: PgExecutor<'e>>(executor: E, link_creditable_attribute_type_attribute_type: i32) -> Result<Vec<LinkAttributeCredit>> {
        query_as::<_, LinkAttributeCredit>(r#"SELECT * FROM "musicbrainz"."link_attribute_credit" WHERE attribute_type = $1"#)
            .bind(link_creditable_attribute_type_attribute_type)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_credit: LinkAttributeCredit) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"INSERT INTO "link_attribute_credit" ("link", "attribute_type", "credited_as") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(link_attribute_credit.link)
            .bind(link_attribute_credit.attribute_type)
            .bind(link_attribute_credit.credited_as)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_credit: LinkAttributeCredit) -> Result<LinkAttributeCredit> {
        query_as::<_, LinkAttributeCredit>(r#"UPDATE "link_attribute_credit" SET "credited_as" = $3 WHERE "link" = 1 AND "attribute_type" = 2 RETURNING *;"#)
            .bind(link_attribute_credit.link)
            .bind(link_attribute_credit.attribute_type)
            .bind(link_attribute_credit.credited_as)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_attribute_credit" WHERE "link" = 1 AND "attribute_type" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
