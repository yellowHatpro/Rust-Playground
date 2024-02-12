#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkAttributeType;

pub struct LinkAttributeTypeSet;

impl LinkAttributeTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, link_attribute_type_id: i32) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE parent = $1"#)
            .bind(link_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_link_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, link_attribute_type_id: i32) -> Result<Vec<LinkAttributeType>> {
        query_as::<_, LinkAttributeType>(r#"SELECT * FROM "musicbrainz"."link_attribute_type" WHERE root = $1"#)
            .bind(link_attribute_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_type: LinkAttributeType) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"INSERT INTO "link_attribute_type" ("id", "parent", "root", "child_order", "gid", "name", "description", "last_updated") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(link_attribute_type.id)
            .bind(link_attribute_type.parent)
            .bind(link_attribute_type.root)
            .bind(link_attribute_type.child_order)
            .bind(link_attribute_type.gid)
            .bind(link_attribute_type.name)
            .bind(link_attribute_type.description)
            .bind(link_attribute_type.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_attribute_type: LinkAttributeType) -> Result<LinkAttributeType> {
        query_as::<_, LinkAttributeType>(r#"UPDATE "link_attribute_type" SET "parent" = $2, "root" = $3, "child_order" = $4, "gid" = $5, "name" = $6, "description" = $7, "last_updated" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(link_attribute_type.id)
            .bind(link_attribute_type.parent)
            .bind(link_attribute_type.root)
            .bind(link_attribute_type.child_order)
            .bind(link_attribute_type.gid)
            .bind(link_attribute_type.name)
            .bind(link_attribute_type.description)
            .bind(link_attribute_type.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_attribute_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
