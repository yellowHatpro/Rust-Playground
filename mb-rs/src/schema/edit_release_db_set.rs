#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditRelease;

pub struct EditReleaseSet;

impl EditReleaseSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_release<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release: i32) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = $1 AND "release" = $2"#)
            .bind(edit)
            .bind(release)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, release_list: Vec<i32>) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = ANY($1) AND "release" = ANY($2)"#)
            .bind(edit_list)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release: i32) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "edit" = $1 AND "release" = $2"#)
            .bind(edit)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<EditRelease>> {
        query_as::<_, EditRelease>(r#"SELECT * FROM "musicbrainz"."edit_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release: EditRelease) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"INSERT INTO "edit_release" ("edit", "release") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_release.edit)
            .bind(edit_release.release)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release: EditRelease) -> Result<EditRelease> {
        query_as::<_, EditRelease>(r#"UPDATE "edit_release" SET  WHERE "edit" = 1 AND "release" = 2 RETURNING *;"#)
            .bind(edit_release.edit)
            .bind(edit_release.release)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_release" WHERE "edit" = 1 AND "release" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
