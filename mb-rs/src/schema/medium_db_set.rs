#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Medium;

pub struct MediumSet;

impl MediumSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "release" = $1"#)
            .bind(release)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, release_list: Vec<i32>) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "release" = ANY($1)"#)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, release: i32) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "release" = $1"#)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = $1"#)
            .bind(position)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, position_list: Vec<i32>) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = ANY($1)"#)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = $1"#)
            .bind(position)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Medium> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = $1"#)
            .bind(position)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, position_list: Vec<i32>) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = ANY($1)"#)
            .bind(position_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, position: i32) -> Result<Option<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE "position" = $1"#)
            .bind(position)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_medium_format_id<'e, E: PgExecutor<'e>>(executor: E, medium_format_id: i32) -> Result<Vec<Medium>> {
        query_as::<_, Medium>(r#"SELECT * FROM "musicbrainz"."medium" WHERE format = $1"#)
            .bind(medium_format_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium: Medium) -> Result<Medium> {
        query_as::<_, Medium>(r#"INSERT INTO "medium" ("id", "release", "position", "format", "name", "edits_pending", "last_updated", "track_count") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(medium.id)
            .bind(medium.release)
            .bind(medium.position)
            .bind(medium.format)
            .bind(medium.name)
            .bind(medium.edits_pending)
            .bind(medium.last_updated)
            .bind(medium.track_count)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium: Medium) -> Result<Medium> {
        query_as::<_, Medium>(r#"UPDATE "medium" SET "release" = $2, "position" = $3, "format" = $4, "name" = $5, "edits_pending" = $6, "last_updated" = $7, "track_count" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(medium.id)
            .bind(medium.release)
            .bind(medium.position)
            .bind(medium.format)
            .bind(medium.name)
            .bind(medium.edits_pending)
            .bind(medium.last_updated)
            .bind(medium.track_count)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
