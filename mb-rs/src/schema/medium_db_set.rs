#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Medium;

pub struct MediumSet;

impl MediumSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_format_id_where_format_is<'e, E: PgExecutor<'e>>(executor: E, medium_format_id: i32) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE format = $1"#)
            .bind(medium_format_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium: Medium) -> Result<Medium> {
        query_as::<_, Medium>(r#"INSERT INTO "medium" ("id", "release", "position", "format", "name", "edits_pending", "last_updated", "track_count") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(medium.last_updated)
            .bind(medium.track_count)
            .bind(medium.format)
            .bind(medium.id)
            .bind(medium.release)
            .bind(medium.position)
            .bind(medium.name)
            .bind(medium.edits_pending)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium: Medium) -> Result<Medium> {
        query_as::<_, Medium>(r#"UPDATE "medium" SET "release" = $2, "position" = $3, "name" = $5, "last_updated" = $7, "edits_pending" = $6, "track_count" = $8, "format" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium.last_updated)
            .bind(medium.id)
            .bind(medium.release)
            .bind(medium.name)
            .bind(medium.format)
            .bind(medium.track_count)
            .bind(medium.edits_pending)
            .bind(medium.position)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
