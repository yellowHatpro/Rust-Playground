#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditGenre;

pub struct EditGenreSet;

impl EditGenreSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_genre<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, genre: i32) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = $1 AND "genre" = $2"#)
            .bind(edit)
            .bind(genre)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_genre_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, genre_list: Vec<i32>) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = ANY($1) AND "genre" = ANY($2)"#)
            .bind(edit_list)
            .bind(genre_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_genre_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, genre: i32) -> Result<Option<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE "edit" = $1 AND "genre" = $2"#)
            .bind(edit)
            .bind(genre)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_id_where_genre_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<EditGenre>> {
        query_as::<_, EditGenre>(r#"SELECT * FROM "musicbrainz"."edit_genre" WHERE genre = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_genre: EditGenre) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"INSERT INTO "edit_genre" ("edit", "genre") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_genre.genre)
            .bind(edit_genre.edit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_genre: EditGenre) -> Result<EditGenre> {
        query_as::<_, EditGenre>(r#"UPDATE "edit_genre" SET  WHERE "genre" = 2 AND "edit" = 1 RETURNING *;"#)
            .bind(edit_genre.edit)
            .bind(edit_genre.genre)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_genre" WHERE "edit" = 1 AND "genre" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
