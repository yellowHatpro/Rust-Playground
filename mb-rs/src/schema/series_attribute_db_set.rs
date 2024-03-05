#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::SeriesAttribute;

pub struct SeriesAttributeSet;

impl SeriesAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<SeriesAttribute> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_series_id_where_series_is<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_attribute_type_id_where_series_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, series_attribute_type_id: i32) -> Result<Vec<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE series_attribute_type = $1"#)
            .bind(series_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_attribute_type_allowed_value_id_where_series_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, series_attribute_type_allowed_value_id: i32) -> Result<Vec<SeriesAttribute>> {
        query_as::<_, SeriesAttribute>(r#"SELECT * FROM "musicbrainz"."series_attribute" WHERE series_attribute_type_allowed_value = $1"#)
            .bind(series_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, series_attribute: SeriesAttribute) -> Result<SeriesAttribute> {
        query_as::<_, SeriesAttribute>(r#"INSERT INTO "series_attribute" ("id", "series", "series_attribute_type", "series_attribute_type_allowed_value", "series_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(series_attribute.series)
            .bind(series_attribute.series_attribute_type_allowed_value)
            .bind(series_attribute.series_attribute_text)
            .bind(series_attribute.id)
            .bind(series_attribute.series_attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, series_attribute: SeriesAttribute) -> Result<SeriesAttribute> {
        query_as::<_, SeriesAttribute>(r#"UPDATE "series_attribute" SET "series" = $2, "series_attribute_type" = $3, "series_attribute_type_allowed_value" = $4, "series_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(series_attribute.id)
            .bind(series_attribute.series_attribute_text)
            .bind(series_attribute.series)
            .bind(series_attribute.series_attribute_type_allowed_value)
            .bind(series_attribute.series_attribute_type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."series_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
