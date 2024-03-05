#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditRelease;

pub struct EditReleaseSet;

impl EditReleaseSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_release<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release: i32) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = $1 AND "release" = $2"#)
            .bind(edit)
            .bind(release)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, release_list: Vec<i32>) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = ANY($1) AND "release" = ANY($2)"#)
            .bind(edit_list)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release: i32) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = $1 AND "release" = $2"#)
            .bind(edit)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release: EditRelease) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"INSERT INTO "edit_release" ("edit", "release") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_release.edit)
            .bind(edit_release.release)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release: EditRelease) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"UPDATE "edit_release" SET  WHERE "edit" = 1 AND "release" = 2 RETURNING *;"#)
            .bind(edit_release.release)
            .bind(edit_release.edit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_release" WHERE "release" = 2 AND "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
