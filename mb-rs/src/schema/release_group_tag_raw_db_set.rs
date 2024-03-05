#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupTagRaw;

pub struct ReleaseGroupTagRawSet;

impl ReleaseGroupTagRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release_group_and_editor_and_tag<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, editor: i32, tag: i32) -> Result<ReleaseGroupTagRaw> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE "release_group" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(release_group)
            .bind(editor)
            .bind(tag)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_group_and_editor_and_tag_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_list: Vec<i32>, editor_list: Vec<i32>, tag_list: Vec<i32>) -> Result<Vec<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE "release_group" = ANY($1) AND "editor" = ANY($2) AND "tag" = ANY($3)"#)
            .bind(release_group_list)
            .bind(editor_list)
            .bind(tag_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_group_and_editor_and_tag_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, editor: i32, tag: i32) -> Result<Option<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE "release_group" = $1 AND "editor" = $2 AND "tag" = $3"#)
            .bind(release_group)
            .bind(editor)
            .bind(tag)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_tag_id_where_tag_is<'e, E: PgExecutor<'e>>(executor: E, tag_id: i32) -> Result<Vec<ReleaseGroupTagRaw>> {
        query_as::<_, ReleaseGroupTagRaw>(r#"SELECT * FROM "musicbrainz"."release_group_tag_raw" WHERE tag = $1"#)
            .bind(tag_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_tag_raw: ReleaseGroupTagRaw) -> Result<ReleaseGroupTagRaw> {
        query_as::<_, ReleaseGroupTagRaw>(r#"INSERT INTO "release_group_tag_raw" ("release_group", "editor", "tag", "is_upvote") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(release_group_tag_raw.tag)
            .bind(release_group_tag_raw.editor)
            .bind(release_group_tag_raw.release_group)
            .bind(release_group_tag_raw.is_upvote)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_tag_raw: ReleaseGroupTagRaw) -> Result<ReleaseGroupTagRaw> {
        query_as::<_, ReleaseGroupTagRaw>(r#"UPDATE "release_group_tag_raw" SET "is_upvote" = $4 WHERE "release_group" = 1 AND "tag" = 3 AND "editor" = 2 RETURNING *;"#)
            .bind(release_group_tag_raw.editor)
            .bind(release_group_tag_raw.tag)
            .bind(release_group_tag_raw.is_upvote)
            .bind(release_group_tag_raw.release_group)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_tag_raw" WHERE "editor" = 2 AND "tag" = 3 AND "release_group" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
