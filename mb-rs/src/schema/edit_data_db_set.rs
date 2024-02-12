#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditData;

pub struct EditDataSet;

impl EditDataSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = $1"#)
            .bind(edit)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = ANY($1)"#)
            .bind(edit_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = $1"#)
            .bind(edit)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_data: EditData) -> Result<EditData> {
        query_as::<_, EditData>(r#"INSERT INTO "edit_data" ("edit", "data") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_data.edit)
            .bind(edit_data.data)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_data: EditData) -> Result<EditData> {
        query_as::<_, EditData>(r#"UPDATE "edit_data" SET "data" = $2 WHERE "edit" = 1 RETURNING *;"#)
            .bind(edit_data.edit)
            .bind(edit_data.data)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_data" WHERE "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
