#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Vote;

pub struct VoteSet;

impl VoteSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, vote: Vote) -> Result<Vote> {
        query_as::<_, Vote>(r#"INSERT INTO "vote" ("id", "editor", "edit", "vote", "vote_time", "superseded") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(vote.editor)
            .bind(vote.edit)
            .bind(vote.id)
            .bind(vote.vote)
            .bind(vote.vote_time)
            .bind(vote.superseded)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, vote: Vote) -> Result<Vote> {
        query_as::<_, Vote>(r#"UPDATE "vote" SET "edit" = $3, "vote" = $4, "superseded" = $6, "vote_time" = $5, "editor" = $2 WHERE "id" = 1 RETURNING *;"#)
            .bind(vote.superseded)
            .bind(vote.id)
            .bind(vote.vote)
            .bind(vote.vote_time)
            .bind(vote.edit)
            .bind(vote.editor)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."vote" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
