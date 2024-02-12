#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupAttribute;

pub struct ReleaseGroupAttributeSet;

impl ReleaseGroupAttributeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, release_group_attribute_type_id: i32) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE release_group_attribute_type = $1"#)
            .bind(release_group_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, release_group_attribute_type_allowed_value_id: i32) -> Result<Vec<ReleaseGroupAttribute>> {
        query_as::<_, ReleaseGroupAttribute>(r#"SELECT * FROM "musicbrainz"."release_group_attribute" WHERE release_group_attribute_type_allowed_value = $1"#)
            .bind(release_group_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute: ReleaseGroupAttribute) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"INSERT INTO "release_group_attribute" ("id", "release_group", "release_group_attribute_type", "release_group_attribute_type_allowed_value", "release_group_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_group_attribute.id)
            .bind(release_group_attribute.release_group)
            .bind(release_group_attribute.release_group_attribute_type)
            .bind(release_group_attribute.release_group_attribute_type_allowed_value)
            .bind(release_group_attribute.release_group_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_attribute: ReleaseGroupAttribute) -> Result<ReleaseGroupAttribute> {
        query_as::<_, ReleaseGroupAttribute>(r#"UPDATE "release_group_attribute" SET "release_group" = $2, "release_group_attribute_type" = $3, "release_group_attribute_type_allowed_value" = $4, "release_group_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_group_attribute.id)
            .bind(release_group_attribute.release_group)
            .bind(release_group_attribute.release_group_attribute_type)
            .bind(release_group_attribute.release_group_attribute_type_allowed_value)
            .bind(release_group_attribute.release_group_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
