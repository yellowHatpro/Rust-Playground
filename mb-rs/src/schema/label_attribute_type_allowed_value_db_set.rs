#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelAttributeTypeAllowedValue;

pub struct LabelAttributeTypeAllowedValueSet;

impl LabelAttributeTypeAllowedValueSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_id: i32) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE label_attribute_type = $1"#)
            .bind(label_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_allowed_value_id: i32) -> Result<Vec<LabelAttributeTypeAllowedValue>> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(label_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute_type_allowed_value: LabelAttributeTypeAllowedValue) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"INSERT INTO "label_attribute_type_allowed_value" ("id", "label_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(label_attribute_type_allowed_value.id)
            .bind(label_attribute_type_allowed_value.label_attribute_type)
            .bind(label_attribute_type_allowed_value.value)
            .bind(label_attribute_type_allowed_value.parent)
            .bind(label_attribute_type_allowed_value.child_order)
            .bind(label_attribute_type_allowed_value.description)
            .bind(label_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute_type_allowed_value: LabelAttributeTypeAllowedValue) -> Result<LabelAttributeTypeAllowedValue> {
        query_as::<_, LabelAttributeTypeAllowedValue>(r#"UPDATE "label_attribute_type_allowed_value" SET "label_attribute_type" = $2, "value" = $3, "parent" = $4, "child_order" = $5, "description" = $6, "gid" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_attribute_type_allowed_value.id)
            .bind(label_attribute_type_allowed_value.label_attribute_type)
            .bind(label_attribute_type_allowed_value.value)
            .bind(label_attribute_type_allowed_value.parent)
            .bind(label_attribute_type_allowed_value.child_order)
            .bind(label_attribute_type_allowed_value.description)
            .bind(label_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
