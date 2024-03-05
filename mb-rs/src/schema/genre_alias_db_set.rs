#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::GenreAlias;

pub struct GenreAliasSet;

impl GenreAliasSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<GenreAlias>> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<GenreAlias> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<GenreAlias>> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<GenreAlias>> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_genre_id_where_genre_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<GenreAlias>> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias" WHERE genre = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_alias_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, genre_alias_type_id: i32) -> Result<Vec<GenreAlias>> {
        query_as::<_, GenreAlias>(r#"SELECT * FROM "musicbrainz"."genre_alias" WHERE type = $1"#)
            .bind(genre_alias_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, genre_alias: GenreAlias) -> Result<GenreAlias> {
        query_as::<_, GenreAlias>(r#"INSERT INTO "genre_alias" ("id", "genre", "name", "locale", "edits_pending", "last_updated", "type", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "primary_for_locale", "ended") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) RETURNING *;"#)
            .bind(genre_alias.locale)
            .bind(genre_alias.primary_for_locale)
            .bind(genre_alias.begin_date_year)
            .bind(genre_alias.last_updated)
            .bind(genre_alias.end_date_month)
            .bind(genre_alias.id)
            .bind(genre_alias.begin_date_month)
            .bind(genre_alias.end_date_day)
            .bind(genre_alias.end_date_year)
            .bind(genre_alias.name)
            .bind(genre_alias.sort_name)
            .bind(genre_alias.ended)
            .bind(genre_alias.Type)
            .bind(genre_alias.genre)
            .bind(genre_alias.edits_pending)
            .bind(genre_alias.begin_date_day)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, genre_alias: GenreAlias) -> Result<GenreAlias> {
        query_as::<_, GenreAlias>(r#"UPDATE "genre_alias" SET "last_updated" = $6, "begin_date_month" = $10, "genre" = $2, "locale" = $4, "end_date_month" = $13, "end_date_day" = $14, "begin_date_year" = $9, "ended" = $16, "primary_for_locale" = $15, "type" = $7, "sort_name" = $8, "edits_pending" = $5, "name" = $3, "end_date_year" = $12, "begin_date_day" = $11 WHERE "id" = 1 RETURNING *;"#)
            .bind(genre_alias.end_date_month)
            .bind(genre_alias.primary_for_locale)
            .bind(genre_alias.edits_pending)
            .bind(genre_alias.last_updated)
            .bind(genre_alias.begin_date_month)
            .bind(genre_alias.genre)
            .bind(genre_alias.end_date_year)
            .bind(genre_alias.locale)
            .bind(genre_alias.end_date_day)
            .bind(genre_alias.begin_date_year)
            .bind(genre_alias.begin_date_day)
            .bind(genre_alias.id)
            .bind(genre_alias.sort_name)
            .bind(genre_alias.ended)
            .bind(genre_alias.name)
            .bind(genre_alias.Type)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."genre_alias" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
