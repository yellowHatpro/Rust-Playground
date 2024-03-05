#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceType;

pub struct PlaceTypeSet;

impl PlaceTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceType>> {
        query_as::<_, PlaceType>(r#"SELECT * FROM "musicbrainz"."place_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<PlaceType> {
        query_as::<_, PlaceType>(r#"SELECT * FROM "musicbrainz"."place_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<PlaceType>> {
        query_as::<_, PlaceType>(r#"SELECT * FROM "musicbrainz"."place_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<PlaceType>> {
        query_as::<_, PlaceType>(r#"SELECT * FROM "musicbrainz"."place_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_place_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, place_type_id: i32) -> Result<Vec<PlaceType>> {
        query_as::<_, PlaceType>(r#"SELECT * FROM "musicbrainz"."place_type" WHERE parent = $1"#)
            .bind(place_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_type: PlaceType) -> Result<PlaceType> {
        query_as::<_, PlaceType>(r#"INSERT INTO "place_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(place_type.parent)
            .bind(place_type.id)
            .bind(place_type.child_order)
            .bind(place_type.description)
            .bind(place_type.gid)
            .bind(place_type.name)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_type: PlaceType) -> Result<PlaceType> {
        query_as::<_, PlaceType>(r#"UPDATE "place_type" SET "gid" = $6, "description" = $5, "child_order" = $4, "parent" = $3, "name" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(place_type.description)
            .bind(place_type.gid)
            .bind(place_type.child_order)
            .bind(place_type.id)
            .bind(place_type.name)
            .bind(place_type.parent)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
