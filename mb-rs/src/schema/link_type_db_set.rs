#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LinkType;

pub struct LinkTypeSet;

impl LinkTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_link_type_id<'e, E: PgExecutor<'e>>(executor: E, link_type_id: i32) -> Result<Vec<LinkType>> {
        query_as::<_, LinkType>(r#"SELECT * FROM "musicbrainz"."link_type" WHERE parent = $1"#)
            .bind(link_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: LinkType) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"INSERT INTO "link_type" ("id", "parent", "child_order", "gid", "entity_type0", "entity_type1", "name", "description", "link_phrase", "reverse_link_phrase", "long_link_phrase", "priority", "last_updated", "is_deprecated", "has_dates", "entity0_cardinality", "entity1_cardinality") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *;"#)
            .bind(link_type.id)
            .bind(link_type.parent)
            .bind(link_type.child_order)
            .bind(link_type.gid)
            .bind(link_type.entity_type0)
            .bind(link_type.entity_type1)
            .bind(link_type.name)
            .bind(link_type.description)
            .bind(link_type.link_phrase)
            .bind(link_type.reverse_link_phrase)
            .bind(link_type.long_link_phrase)
            .bind(link_type.priority)
            .bind(link_type.last_updated)
            .bind(link_type.is_deprecated)
            .bind(link_type.has_dates)
            .bind(link_type.entity0_cardinality)
            .bind(link_type.entity1_cardinality)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, link_type: LinkType) -> Result<LinkType> {
        query_as::<_, LinkType>(r#"UPDATE "link_type" SET "parent" = $2, "child_order" = $3, "gid" = $4, "entity_type0" = $5, "entity_type1" = $6, "name" = $7, "description" = $8, "link_phrase" = $9, "reverse_link_phrase" = $10, "long_link_phrase" = $11, "priority" = $12, "last_updated" = $13, "is_deprecated" = $14, "has_dates" = $15, "entity0_cardinality" = $16, "entity1_cardinality" = $17 WHERE "id" = 1 RETURNING *;"#)
            .bind(link_type.id)
            .bind(link_type.parent)
            .bind(link_type.child_order)
            .bind(link_type.gid)
            .bind(link_type.entity_type0)
            .bind(link_type.entity_type1)
            .bind(link_type.name)
            .bind(link_type.description)
            .bind(link_type.link_phrase)
            .bind(link_type.reverse_link_phrase)
            .bind(link_type.long_link_phrase)
            .bind(link_type.priority)
            .bind(link_type.last_updated)
            .bind(link_type.is_deprecated)
            .bind(link_type.has_dates)
            .bind(link_type.entity0_cardinality)
            .bind(link_type.entity1_cardinality)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."link_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
