#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Gender;

pub struct GenderSet;

impl GenderSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_gender_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE parent = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, gender: Gender) -> Result<Gender> {
        query_as::<_, Gender>(r#"INSERT INTO "gender" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(gender.gid)
            .bind(gender.child_order)
            .bind(gender.id)
            .bind(gender.parent)
            .bind(gender.name)
            .bind(gender.description)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, gender: Gender) -> Result<Gender> {
        query_as::<_, Gender>(r#"UPDATE "gender" SET "parent" = $3, "description" = $5, "gid" = $6, "name" = $2, "child_order" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(gender.id)
            .bind(gender.child_order)
            .bind(gender.gid)
            .bind(gender.parent)
            .bind(gender.name)
            .bind(gender.description)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."gender" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
