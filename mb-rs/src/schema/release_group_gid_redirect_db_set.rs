#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleaseGroupGidRedirect;

pub struct ReleaseGroupGidRedirectSet;

impl ReleaseGroupGidRedirectSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleaseGroupGidRedirect>> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"SELECT * FROM "musicbrainz"."release_group_gid_redirect""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_gid<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<ReleaseGroupGidRedirect> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"SELECT * FROM "musicbrainz"."release_group_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, gid_list: Vec<uuid::Uuid>) -> Result<Vec<ReleaseGroupGidRedirect>> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"SELECT * FROM "musicbrainz"."release_group_gid_redirect" WHERE "gid" = ANY($1)"#)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, gid: uuid::Uuid) -> Result<Option<ReleaseGroupGidRedirect>> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"SELECT * FROM "musicbrainz"."release_group_gid_redirect" WHERE "gid" = $1"#)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_release_group_id_where_new_id_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<ReleaseGroupGidRedirect>> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"SELECT * FROM "musicbrainz"."release_group_gid_redirect" WHERE new_id = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_gid_redirect: ReleaseGroupGidRedirect) -> Result<ReleaseGroupGidRedirect> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"INSERT INTO "release_group_gid_redirect" ("gid", "new_id", "created") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(release_group_gid_redirect.gid)
            .bind(release_group_gid_redirect.new_id)
            .bind(release_group_gid_redirect.created)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_group_gid_redirect: ReleaseGroupGidRedirect) -> Result<ReleaseGroupGidRedirect> {
        query_as::<_, ReleaseGroupGidRedirect>(r#"UPDATE "release_group_gid_redirect" SET "new_id" = $2, "created" = $3 WHERE "gid" = 1 RETURNING *;"#)
            .bind(release_group_gid_redirect.gid)
            .bind(release_group_gid_redirect.created)
            .bind(release_group_gid_redirect.new_id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_group_gid_redirect" WHERE "gid" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
