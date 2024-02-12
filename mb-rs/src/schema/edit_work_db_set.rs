#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditWork;

pub struct EditWorkSet;

impl EditWorkSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_work<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, work: i32) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "edit" = $1 AND "work" = $2"#)
            .bind(edit)
            .bind(work)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_work_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, work_list: Vec<i32>) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "edit" = ANY($1) AND "work" = ANY($2)"#)
            .bind(edit_list)
            .bind(work_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_work_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, work: i32) -> Result<Option<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "edit" = $1 AND "work" = $2"#)
            .bind(edit)
            .bind(work)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<EditWork>> {
        query_as::<_, EditWork>(r#"SELECT * FROM "musicbrainz"."edit_work" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_work: EditWork) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"INSERT INTO "edit_work" ("edit", "work") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_work.edit)
            .bind(edit_work.work)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_work: EditWork) -> Result<EditWork> {
        query_as::<_, EditWork>(r#"UPDATE "edit_work" SET  WHERE "edit" = 1 AND "work" = 2 RETURNING *;"#)
            .bind(edit_work.edit)
            .bind(edit_work.work)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_work" WHERE "edit" = 1 AND "work" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
