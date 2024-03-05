#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceAttribute;

pub struct PlaceAttributeSet;

impl PlaceAttributeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<PlaceAttribute> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_id_where_place_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_attribute_type_id_where_place_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, place_attribute_type_id: i32) -> Result<Vec<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE place_attribute_type = $1"#)
            .bind(place_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_attribute_type_allowed_value_id_where_place_attribute_type_allowed_value_is<'e, E: PgExecutor<'e>>(executor: E, place_attribute_type_allowed_value_id: i32) -> Result<Vec<PlaceAttribute>> {
        query_as::<_, PlaceAttribute>(r#"SELECT * FROM "musicbrainz"."place_attribute" WHERE place_attribute_type_allowed_value = $1"#)
            .bind(place_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_attribute: PlaceAttribute) -> Result<PlaceAttribute> {
        query_as::<_, PlaceAttribute>(r#"INSERT INTO "place_attribute" ("id", "place", "place_attribute_type", "place_attribute_type_allowed_value", "place_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(place_attribute.id)
            .bind(place_attribute.place)
            .bind(place_attribute.place_attribute_type_allowed_value)
            .bind(place_attribute.place_attribute_text)
            .bind(place_attribute.place_attribute_type)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_attribute: PlaceAttribute) -> Result<PlaceAttribute> {
        query_as::<_, PlaceAttribute>(r#"UPDATE "place_attribute" SET "place_attribute_type_allowed_value" = $4, "place" = $2, "place_attribute_type" = $3, "place_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(place_attribute.place_attribute_type)
            .bind(place_attribute.place_attribute_type_allowed_value)
            .bind(place_attribute.id)
            .bind(place_attribute.place_attribute_text)
            .bind(place_attribute.place)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
