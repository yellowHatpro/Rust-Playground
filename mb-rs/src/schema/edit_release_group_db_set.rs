#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditReleaseGroup;

pub struct EditReleaseGroupSet;

impl EditReleaseGroupSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditReleaseGroup>> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_release_group<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release_group: i32) -> Result<EditReleaseGroup> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group" WHERE "edit" = $1 AND "release_group" = $2"#)
            .bind(edit)
            .bind(release_group)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_release_group_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, release_group_list: Vec<i32>) -> Result<Vec<EditReleaseGroup>> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group" WHERE "edit" = ANY($1) AND "release_group" = ANY($2)"#)
            .bind(edit_list)
            .bind(release_group_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_release_group_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, release_group: i32) -> Result<Option<EditReleaseGroup>> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group" WHERE "edit" = $1 AND "release_group" = $2"#)
            .bind(edit)
            .bind(release_group)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditReleaseGroup>> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<EditReleaseGroup>> {
        query_as::<_, EditReleaseGroup>(r#"SELECT * FROM "musicbrainz"."edit_release_group" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release_group: EditReleaseGroup) -> Result<EditReleaseGroup> {
        query_as::<_, EditReleaseGroup>(r#"INSERT INTO "edit_release_group" ("edit", "release_group") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_release_group.edit)
            .bind(edit_release_group.release_group)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_release_group: EditReleaseGroup) -> Result<EditReleaseGroup> {
        query_as::<_, EditReleaseGroup>(r#"UPDATE "edit_release_group" SET  WHERE "edit" = 1 AND "release_group" = 2 RETURNING *;"#)
            .bind(edit_release_group.release_group)
            .bind(edit_release_group.edit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_release_group" WHERE "release_group" = 2 AND "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
