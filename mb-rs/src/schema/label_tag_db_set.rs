#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelTag;

pub struct LabelTagSet;

impl LabelTagSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, tag: i32) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "label" = $1 AND "tag" = $2"#)
            .bind(label)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_label_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, label_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "label" = ANY($1) AND "tag" = ANY($2)"#)
            .bind(label_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_label_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, label: i32, tag: i32) -> Result<Option<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "label" = $1 AND "tag" = $2"#)
            .bind(label)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<LabelTag>> {
        query_as::<_, LabelTag>(r#"SELECT * FROM "musicbrainz"."label_tag" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_tag: LabelTag) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"INSERT INTO "label_tag" ("label", "tag", "count", "last_updated") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(label_tag.label)
            .bind(label_tag.tag)
            .bind(label_tag.count)
            .bind(label_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_tag: LabelTag) -> Result<LabelTag> {
        query_as::<_, LabelTag>(r#"UPDATE "label_tag" SET "count" = $3, "last_updated" = $4 WHERE "label" = 1 AND "tag" = 2 RETURNING *;"#)
            .bind(label_tag.label)
            .bind(label_tag.tag)
            .bind(label_tag.count)
            .bind(label_tag.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_tag" WHERE "label" = 1 AND "tag" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
