#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceAttributeTypeAllowedValue;

pub struct PlaceAttributeTypeAllowedValueSet;

impl PlaceAttributeTypeAllowedValueSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceAttributeTypeAllowedValue>> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<PlaceAttributeTypeAllowedValue> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<PlaceAttributeTypeAllowedValue>> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<PlaceAttributeTypeAllowedValue>> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_attribute_type_id_where_place_attribute_type_is<'e, E: PgExecutor<'e>>(executor: E, place_attribute_type_id: i32) -> Result<Vec<PlaceAttributeTypeAllowedValue>> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE place_attribute_type = $1"#)
            .bind(place_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_attribute_type_allowed_value_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, place_attribute_type_allowed_value_id: i32) -> Result<Vec<PlaceAttributeTypeAllowedValue>> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(place_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_attribute_type_allowed_value: PlaceAttributeTypeAllowedValue) -> Result<PlaceAttributeTypeAllowedValue> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"INSERT INTO "place_attribute_type_allowed_value" ("id", "place_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(place_attribute_type_allowed_value.value)
            .bind(place_attribute_type_allowed_value.place_attribute_type)
            .bind(place_attribute_type_allowed_value.id)
            .bind(place_attribute_type_allowed_value.child_order)
            .bind(place_attribute_type_allowed_value.parent)
            .bind(place_attribute_type_allowed_value.description)
            .bind(place_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_attribute_type_allowed_value: PlaceAttributeTypeAllowedValue) -> Result<PlaceAttributeTypeAllowedValue> {
        query_as::<_, PlaceAttributeTypeAllowedValue>(r#"UPDATE "place_attribute_type_allowed_value" SET "child_order" = $5, "parent" = $4, "value" = $3, "description" = $6, "place_attribute_type" = $2, "gid" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(place_attribute_type_allowed_value.place_attribute_type)
            .bind(place_attribute_type_allowed_value.child_order)
            .bind(place_attribute_type_allowed_value.gid)
            .bind(place_attribute_type_allowed_value.description)
            .bind(place_attribute_type_allowed_value.parent)
            .bind(place_attribute_type_allowed_value.id)
            .bind(place_attribute_type_allowed_value.value)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
