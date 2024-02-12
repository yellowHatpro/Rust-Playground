#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelAttribute;

pub struct LabelAttributeSet;

impl LabelAttributeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_label_id<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label_attribute_type = $1"#)
            .bind(label_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_allowed_value_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label_attribute_type_allowed_value = $1"#)
            .bind(label_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute: LabelAttribute) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"INSERT INTO "label_attribute" ("id", "label", "label_attribute_type", "label_attribute_type_allowed_value", "label_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(label_attribute.id)
            .bind(label_attribute.label)
            .bind(label_attribute.label_attribute_type)
            .bind(label_attribute.label_attribute_type_allowed_value)
            .bind(label_attribute.label_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute: LabelAttribute) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"UPDATE "label_attribute" SET "label" = $2, "label_attribute_type" = $3, "label_attribute_type_allowed_value" = $4, "label_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_attribute.id)
            .bind(label_attribute.label)
            .bind(label_attribute.label_attribute_type)
            .bind(label_attribute.label_attribute_type_allowed_value)
            .bind(label_attribute.label_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
