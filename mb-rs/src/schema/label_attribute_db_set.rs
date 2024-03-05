#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LabelAttribute;

pub struct LabelAttributeSet;

impl LabelAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_attribute_type_id_where_label_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label_attribute_type = $1"#)
            .bind(label_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_attribute_type_allowed_value_id_where_label_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, label_attribute_type_allowed_value_id: i32) -> Result<Vec<LabelAttribute>> {
        query_as::<_, LabelAttribute>(r#"SELECT * FROM "musicbrainz"."label_attribute" WHERE label_attribute_type_allowed_value = $1"#)
            .bind(label_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute: LabelAttribute) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"INSERT INTO "label_attribute" ("id", "label", "label_attribute_type", "label_attribute_type_allowed_value", "label_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(label_attribute.label_attribute_type_allowed_value)
            .bind(label_attribute.label_attribute_type)
            .bind(label_attribute.id)
            .bind(label_attribute.label)
            .bind(label_attribute.label_attribute_text)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, label_attribute: LabelAttribute) -> Result<LabelAttribute> {
        query_as::<_, LabelAttribute>(r#"UPDATE "label_attribute" SET "label_attribute_type_allowed_value" = $4, "label_attribute_text" = $5, "label_attribute_type" = $3, "label" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(label_attribute.label_attribute_type)
            .bind(label_attribute.label_attribute_type_allowed_value)
            .bind(label_attribute.id)
            .bind(label_attribute.label)
            .bind(label_attribute.label_attribute_text)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."label_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
