#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumAttribute;

pub struct MediumAttributeSet;

impl MediumAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<MediumAttribute> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_medium_id_where_medium_is<'e, E: PgExecutor<'e>>(executor: E, medium_id: i32) -> Result<Vec<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE medium = $1"#)
            .bind(medium_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_attribute_type_id_where_medium_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_id: i32) -> Result<Vec<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE medium_attribute_type = $1"#)
            .bind(medium_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_attribute_type_allowed_value_id_where_medium_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, medium_attribute_type_allowed_value_id: i32) -> Result<Vec<MediumAttribute>> {
        query_as::<_, MediumAttribute>(r#"SELECT * FROM "musicbrainz"."medium_attribute" WHERE medium_attribute_type_allowed_value = $1"#)
            .bind(medium_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute: MediumAttribute) -> Result<MediumAttribute> {
        query_as::<_, MediumAttribute>(r#"INSERT INTO "medium_attribute" ("id", "medium", "medium_attribute_type", "medium_attribute_type_allowed_value", "medium_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(medium_attribute.medium)
            .bind(medium_attribute.medium_attribute_type)
            .bind(medium_attribute.medium_attribute_type_allowed_value)
            .bind(medium_attribute.medium_attribute_text)
            .bind(medium_attribute.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_attribute: MediumAttribute) -> Result<MediumAttribute> {
        query_as::<_, MediumAttribute>(r#"UPDATE "medium_attribute" SET "medium_attribute_type" = $3, "medium_attribute_type_allowed_value" = $4, "medium_attribute_text" = $5, "medium" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium_attribute.medium_attribute_text)
            .bind(medium_attribute.medium)
            .bind(medium_attribute.medium_attribute_type_allowed_value)
            .bind(medium_attribute.id)
            .bind(medium_attribute.medium_attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
