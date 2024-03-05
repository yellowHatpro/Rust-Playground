#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseMeta;

pub struct ReleaseMetaSet;

impl ReleaseMetaSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseMeta>> {
        query_as::<_, ReleaseMeta>(r#"SELECT * FROM "musicbrainz"."release_meta""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseMeta> {
        query_as::<_, ReleaseMeta>(r#"SELECT * FROM "musicbrainz"."release_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseMeta>> {
        query_as::<_, ReleaseMeta>(r#"SELECT * FROM "musicbrainz"."release_meta" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseMeta>> {
        query_as::<_, ReleaseMeta>(r#"SELECT * FROM "musicbrainz"."release_meta" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_id_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseMeta>> {
        query_as::<_, ReleaseMeta>(r#"SELECT * FROM "musicbrainz"."release_meta" WHERE id = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_meta: ReleaseMeta) -> Result<ReleaseMeta> {
        query_as::<_, ReleaseMeta>(r#"INSERT INTO "release_meta" ("id", "date_added", "info_url", "amazon_asin", "cover_art_presence") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_meta.info_url)
            .bind(release_meta.amazon_asin)
            .bind(release_meta.id)
            .bind(release_meta.date_added)
            .bind(release_meta.cover_art_presence)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_meta: ReleaseMeta) -> Result<ReleaseMeta> {
        query_as::<_, ReleaseMeta>(r#"UPDATE "release_meta" SET "cover_art_presence" = $5, "info_url" = $3, "date_added" = $2, "amazon_asin" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_meta.date_added)
            .bind(release_meta.cover_art_presence)
            .bind(release_meta.id)
            .bind(release_meta.amazon_asin)
            .bind(release_meta.info_url)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_meta" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
