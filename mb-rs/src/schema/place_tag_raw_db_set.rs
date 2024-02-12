#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::PlaceTagRaw;

pub struct PlaceTagRawSet;

impl PlaceTagRawSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_place_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, editor: i32, tag: i32) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "place" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(place)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_place_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, place_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "place" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(place_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_place_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, place: i32, editor: i32, tag: i32) -> Result<Option<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "place" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(place)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_place_id<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<PlaceTagRaw>> {
        query_as::<_, PlaceTagRaw>(r#"SELECT * FROM "musicbrainz"."place_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag_raw: PlaceTagRaw) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"INSERT INTO "place_tag_raw" ("place", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(place_tag_raw.place)
            .bind(place_tag_raw.editor)
            .bind(place_tag_raw.tag)
            .bind(place_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, place_tag_raw: PlaceTagRaw) -> Result<PlaceTagRaw> {
        query_as::<_, PlaceTagRaw>(r#"UPDATE "place_tag_raw" SET "is_upvote" = $4 WHERE "place" = 1 AND "editor" = 2 AND "tag" = 3 RETURNING *;"#)
            .bind(place_tag_raw.place)
            .bind(place_tag_raw.editor)
            .bind(place_tag_raw.tag)
            .bind(place_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."place_tag_raw" WHERE "place" = 1 AND "editor" = 2 AND "tag" = 3"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
