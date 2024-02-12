#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditMood;

pub struct EditMoodSet;

impl EditMoodSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_mood<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, mood: i32) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = $1 AND "mood" = $2"#)
            .bind(edit)
            .bind(mood)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_mood_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, mood_list: Vec<i32>) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = ANY($1) AND "mood" = ANY($2)"#)
            .bind(edit_list)
            .bind(mood_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_mood_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, mood: i32) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "edit" = $1 AND "mood" = $2"#)
            .bind(edit)
            .bind(mood)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_mood_id<'e, E: PgExecutor<'e>>(executor: E, mood_id: i32) -> Result<Vec<EditMood>> {
        query_as::<_, EditMood>(r#"SELECT * FROM "musicbrainz"."edit_mood" WHERE mood = $1"#)
            .bind(mood_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_mood: EditMood) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"INSERT INTO "edit_mood" ("edit", "mood") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_mood.edit)
            .bind(edit_mood.mood)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_mood: EditMood) -> Result<EditMood> {
        query_as::<_, EditMood>(r#"UPDATE "edit_mood" SET  WHERE "edit" = 1 AND "mood" = 2 RETURNING *;"#)
            .bind(edit_mood.edit)
            .bind(edit_mood.mood)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_mood" WHERE "edit" = 1 AND "mood" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
