#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditGenre;

pub struct EditGenreSet;

impl EditGenreSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_genre<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, genre: i32) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = $1 AND "genre" = $2"#)
            .bind(edit)
            .bind(genre)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_genre_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, genre_list: Vec<i32>) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = ANY($1) AND "genre" = ANY($2)"#)
            .bind(edit_list)
            .bind(genre_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_genre_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, genre: i32) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = $1 AND "genre" = $2"#)
            .bind(edit)
            .bind(genre)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_id<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE genre = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_genre: EditGenre) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"INSERT INTO "edit_genre" ("edit", "genre") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_genre.edit)
            .bind(edit_genre.genre)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_genre: EditGenre) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"UPDATE "edit_genre" SET  WHERE "edit" = 1 AND "genre" = 2 RETURNING *;"#)
            .bind(edit_genre.edit)
            .bind(edit_genre.genre)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_genre" WHERE "edit" = 1 AND "genre" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
