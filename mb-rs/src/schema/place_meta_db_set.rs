#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceMeta;

pub struct PlaceMetaSet;

impl PlaceMetaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_place_id<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceMeta>> {
        query_as::<_, PlaceMeta>(r#"SELECT * FROM "musicbrainz"."place_meta" WHERE id = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_meta: PlaceMeta) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"INSERT INTO "place_meta" ("id", "rating", "rating_count") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(place_meta.id)
            .bind(place_meta.rating)
            .bind(place_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_meta: PlaceMeta) -> Result<PlaceMeta> {
        query_as::<_, PlaceMeta>(r#"UPDATE "place_meta" SET "rating" = $2, "rating_count" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(place_meta.id)
            .bind(place_meta.rating)
            .bind(place_meta.rating_count)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
