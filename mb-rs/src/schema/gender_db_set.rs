#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Gender;

pub struct GenderSet;

impl GenderSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Gender> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_gender_id<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Gender>> {
        query_as::<_, Gender>(r#"SELECT * FROM "musicbrainz"."gender" WHERE parent = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, gender: Gender) -> Result<Gender> {
        query_as::<_, Gender>(r#"INSERT INTO "gender" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(gender.id)
            .bind(gender.name)
            .bind(gender.parent)
            .bind(gender.child_order)
            .bind(gender.description)
            .bind(gender.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, gender: Gender) -> Result<Gender> {
        query_as::<_, Gender>(r#"UPDATE "gender" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(gender.id)
            .bind(gender.name)
            .bind(gender.parent)
            .bind(gender.child_order)
            .bind(gender.description)
            .bind(gender.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."gender" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
