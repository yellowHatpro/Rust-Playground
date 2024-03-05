#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::TagRelation;

pub struct TagRelationSet;

impl TagRelationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<TagRelation>> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_tag1_and_tag2<'e, E: PgExecutor<'e>>(&self, executor: E, tag1: i32, tag2: i32) -> Result<TagRelation> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation" WHERE "tag1" = $1 AND "tag2" = $2"#)
            .bind(tag1)
            .bind(tag2)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_tag1_and_tag2_list<'e, E: PgExecutor<'e>>(&self, executor: E, tag1_list: Vec<i32>, tag2_list: Vec<i32>) -> Result<Vec<TagRelation>> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation" WHERE "tag1" = ANY($1) AND "tag2" = ANY($2)"#)
            .bind(tag1_list)
            .bind(tag2_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_tag1_and_tag2_optional<'e, E: PgExecutor<'e>>(&self, executor: E, tag1: i32, tag2: i32) -> Result<Option<TagRelation>> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation" WHERE "tag1" = $1 AND "tag2" = $2"#)
            .bind(tag1)
            .bind(tag2)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_tag_id_where_tag1_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<TagRelation>> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation" WHERE tag1 = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag2_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<TagRelation>> {
        query_as::<_, TagRelation>(r#"SELECT * FROM "musicbrainz"."tag_relation" WHERE tag2 = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, tag_relation: TagRelation) -> Result<TagRelation> {
        query_as::<_, TagRelation>(r#"INSERT INTO "tag_relation" ("tag1", "tag2", "weight", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(tag_relation.tag1)
            .bind(tag_relation.last_updated)
            .bind(tag_relation.weight)
            .bind(tag_relation.tag2)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, tag_relation: TagRelation) -> Result<TagRelation> {
        query_as::<_, TagRelation>(r#"UPDATE "tag_relation" SET "last_updated" = $4, "weight" = $3 WHERE "tag2" = 2 AND "tag1" = 1 RETURNING *;"#)
            .bind(tag_relation.tag2)
            .bind(tag_relation.weight)
            .bind(tag_relation.last_updated)
            .bind(tag_relation.tag1)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."tag_relation" WHERE "tag2" = 2 AND "tag1" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
