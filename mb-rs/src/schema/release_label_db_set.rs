#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseLabel;

pub struct ReleaseLabelSet;

impl ReleaseLabelSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseLabel>> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleaseLabel> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleaseLabel>> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleaseLabel>> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<ReleaseLabel>> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<ReleaseLabel>> {
        query_as::<_, ReleaseLabel>(r#"SELECT * FROM "musicbrainz"."release_label" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_label: ReleaseLabel) -> Result<ReleaseLabel> {
        query_as::<_, ReleaseLabel>(r#"INSERT INTO "release_label" ("id", "release", "label", "catalog_number", "last_updated") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(release_label.id)
            .bind(release_label.catalog_number)
            .bind(release_label.release)
            .bind(release_label.last_updated)
            .bind(release_label.label)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_label: ReleaseLabel) -> Result<ReleaseLabel> {
        query_as::<_, ReleaseLabel>(r#"UPDATE "release_label" SET "catalog_number" = $4, "last_updated" = $5, "release" = $2, "label" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_label.release)
            .bind(release_label.last_updated)
            .bind(release_label.id)
            .bind(release_label.catalog_number)
            .bind(release_label.label)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_label" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
