#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AreaTagRaw;

pub struct AreaTagRawSet;

impl AreaTagRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_area_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, editor: i32, tag: i32) -> Result<AreaTagRaw> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE "area" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(area)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_area_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, area_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE "area" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(area_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_area_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, area: i32, editor: i32, tag: i32) -> Result<Option<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE "area" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(area)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<AreaTagRaw>> {
        query_as::<_, AreaTagRaw>(r#"SELECT * FROM "musicbrainz"."area_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, area_tag_raw: AreaTagRaw) -> Result<AreaTagRaw> {
        query_as::<_, AreaTagRaw>(r#"INSERT INTO "area_tag_raw" ("area", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(area_tag_raw.tag)
            .bind(area_tag_raw.editor)
            .bind(area_tag_raw.area)
            .bind(area_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, area_tag_raw: AreaTagRaw) -> Result<AreaTagRaw> {
        query_as::<_, AreaTagRaw>(r#"UPDATE "area_tag_raw" SET "is_upvote" = $4 WHERE "tag" = 3 AND "area" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(area_tag_raw.tag)
            .bind(area_tag_raw.is_upvote)
            .bind(area_tag_raw.area)
            .bind(area_tag_raw.editor)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."area_tag_raw" WHERE "tag" = 3 AND "area" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
