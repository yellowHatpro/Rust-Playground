#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditMood;

pub struct EditMoodSet;

impl EditMoodSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_mood<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, mood: i32) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = $1 AND "mood" = $2"#)
            .bind(edit)
            .bind(mood)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_mood_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, mood_list: Vec<i32>) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = ANY($1) AND "mood" = ANY($2)"#)
            .bind(edit_list)
            .bind(mood_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_mood_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, mood: i32) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = $1 AND "mood" = $2"#)
            .bind(edit)
            .bind(mood)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_mood_id_where_mood_is<'e, E: PgExecutor<'e>>(executor: E, mood_id: i32) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE mood = $1"#)
            .bind(mood_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_mood: EditMood) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"INSERT INTO "edit_mood" ("edit", "mood") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_mood.mood)
            .bind(edit_mood.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_mood: EditMood) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"UPDATE "edit_mood" SET  WHERE "edit" = 1 AND "mood" = 2 RETURNING *;"#)
            .bind(edit_mood.edit)
            .bind(edit_mood.mood)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_mood" WHERE "edit" = 1 AND "mood" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
