#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseRaw;

pub struct ReleaseRawSet;

impl ReleaseRawSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseRaw>> {
        query_as::<_, ReleaseRaw>(r#"SELECT * FROM "musicbrainz"."release_raw" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_raw: ReleaseRaw) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"INSERT INTO "release_raw" ("id", "title", "artist", "added", "last_modified", "lookup_count", "modify_count", "source", "barcode", "comment") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;"#)
            .bind(release_raw.title)
            .bind(release_raw.artist)
            .bind(release_raw.added)
            .bind(release_raw.id)
            .bind(release_raw.source)
            .bind(release_raw.barcode)
            .bind(release_raw.comment)
            .bind(release_raw.lookup_count)
            .bind(release_raw.modify_count)
            .bind(release_raw.last_modified)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_raw: ReleaseRaw) -> Result<ReleaseRaw> {
        query_as::<_, ReleaseRaw>(r#"UPDATE "release_raw" SET "comment" = $10, "lookup_count" = $6, "modify_count" = $7, "source" = $8, "artist" = $3, "last_modified" = $5, "barcode" = $9, "added" = $4, "title" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_raw.barcode)
            .bind(release_raw.comment)
            .bind(release_raw.title)
            .bind(release_raw.lookup_count)
            .bind(release_raw.last_modified)
            .bind(release_raw.id)
            .bind(release_raw.artist)
            .bind(release_raw.added)
            .bind(release_raw.modify_count)
            .bind(release_raw.source)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_raw" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
