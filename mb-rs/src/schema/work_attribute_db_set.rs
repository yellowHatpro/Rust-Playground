#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::WorkAttribute;

pub struct WorkAttributeSet;

impl WorkAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<WorkAttribute> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_attribute_type_id_where_work_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, work_attribute_type_id: i32) -> Result<Vec<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE work_attribute_type = $1"#)
            .bind(work_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_attribute_type_allowed_value_id_where_work_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, work_attribute_type_allowed_value_id: i32) -> Result<Vec<WorkAttribute>> {
        query_as::<_, WorkAttribute>(r#"SELECT * FROM "musicbrainz"."work_attribute" WHERE work_attribute_type_allowed_value = $1"#)
            .bind(work_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, work_attribute: WorkAttribute) -> Result<WorkAttribute> {
        query_as::<_, WorkAttribute>(r#"INSERT INTO "work_attribute" ("id", "work", "work_attribute_type", "work_attribute_type_allowed_value", "work_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(work_attribute.work_attribute_type)
            .bind(work_attribute.work_attribute_type_allowed_value)
            .bind(work_attribute.id)
            .bind(work_attribute.work_attribute_text)
            .bind(work_attribute.work)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, work_attribute: WorkAttribute) -> Result<WorkAttribute> {
        query_as::<_, WorkAttribute>(r#"UPDATE "work_attribute" SET "work" = $2, "work_attribute_type_allowed_value" = $4, "work_attribute_text" = $5, "work_attribute_type" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(work_attribute.work_attribute_type)
            .bind(work_attribute.work)
            .bind(work_attribute.work_attribute_type_allowed_value)
            .bind(work_attribute.id)
            .bind(work_attribute.work_attribute_text)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."work_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
