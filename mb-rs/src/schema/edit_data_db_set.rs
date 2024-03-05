#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditData;

pub struct EditDataSet;

impl EditDataSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32) -> Result<EditData> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = $1"#)
            .bind(edit)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = ANY($1)"#)
            .bind(edit_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32) -> Result<Option<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE "edit" = $1"#)
            .bind(edit)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditData>> {
        query_as::<_, EditData>(r#"SELECT * FROM "musicbrainz"."edit_data" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_data: EditData) -> Result<EditData> {
        query_as::<_, EditData>(r#"INSERT INTO "edit_data" ("edit", "data") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_data.data)
            .bind(edit_data.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_data: EditData) -> Result<EditData> {
        query_as::<_, EditData>(r#"UPDATE "edit_data" SET "data" = $2 WHERE "edit" = 1 RETURNING *;"#)
            .bind(edit_data.data)
            .bind(edit_data.edit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_data" WHERE "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
