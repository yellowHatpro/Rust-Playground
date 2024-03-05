#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupRatingRaw;

pub struct ReleaseGroupRatingRawSet;

impl ReleaseGroupRatingRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupRatingRaw>> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_release_group_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, editor: i32) -> Result<ReleaseGroupRatingRaw> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw" WHERE "release_group" = $1 AND "editor" = $2"#)
            .bind(release_group)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_release_group_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<ReleaseGroupRatingRaw>> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw" WHERE "release_group" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(release_group_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_release_group_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release_group: i32, editor: i32) -> Result<Option<ReleaseGroupRatingRaw>> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw" WHERE "release_group" = $1 AND "editor" = $2"#)
            .bind(release_group)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupRatingRaw>> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<ReleaseGroupRatingRaw>> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"SELECT * FROM "musicbrainz"."release_group_rating_raw" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_rating_raw: ReleaseGroupRatingRaw) -> Result<ReleaseGroupRatingRaw> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"INSERT INTO "release_group_rating_raw" ("release_group", "editor", "rating") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(release_group_rating_raw.rating)
            .bind(release_group_rating_raw.release_group)
            .bind(release_group_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_rating_raw: ReleaseGroupRatingRaw) -> Result<ReleaseGroupRatingRaw> {
        query_as::<_, ReleaseGroupRatingRaw>(r#"UPDATE "release_group_rating_raw" SET "rating" = $3 WHERE "editor" = 2 AND "release_group" = 1 RETURNING *;"#)
            .bind(release_group_rating_raw.rating)
            .bind(release_group_rating_raw.release_group)
            .bind(release_group_rating_raw.editor)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_rating_raw" WHERE "release_group" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
