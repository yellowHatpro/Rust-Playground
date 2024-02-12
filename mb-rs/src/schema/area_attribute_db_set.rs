#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaAttribute;

pub struct AreaAttributeSet;

impl AreaAttributeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, area_attribute_type_id: i32) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE area_attribute_type = $1"#)
            .bind(area_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, area_attribute_type_allowed_value_id: i32) -> Result<Vec<AreaAttribute>> {
        query_as::<_, AreaAttribute>(r#"SELECT * FROM "musicbrainz"."area_attribute" WHERE area_attribute_type_allowed_value = $1"#)
            .bind(area_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute: AreaAttribute) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"INSERT INTO "area_attribute" ("id", "area", "area_attribute_type", "area_attribute_type_allowed_value", "area_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(area_attribute.id)
            .bind(area_attribute.area)
            .bind(area_attribute.area_attribute_type)
            .bind(area_attribute.area_attribute_type_allowed_value)
            .bind(area_attribute.area_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_attribute: AreaAttribute) -> Result<AreaAttribute> {
        query_as::<_, AreaAttribute>(r#"UPDATE "area_attribute" SET "area" = $2, "area_attribute_type" = $3, "area_attribute_type_allowed_value" = $4, "area_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(area_attribute.id)
            .bind(area_attribute.area)
            .bind(area_attribute.area_attribute_type)
            .bind(area_attribute.area_attribute_type_allowed_value)
            .bind(area_attribute.area_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
