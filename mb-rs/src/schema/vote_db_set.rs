#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Vote;

pub struct VoteSet;

impl VoteSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vote> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<Vote>> {
        query_as::<_, Vote>(r#"SELECT * FROM "musicbrainz"."vote" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, vote: Vote) -> Result<Vote> {
        query_as::<_, Vote>(r#"INSERT INTO "vote" ("id", "editor", "edit", "vote", "vote_time", "superseded") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(vote.id)
            .bind(vote.editor)
            .bind(vote.edit)
            .bind(vote.vote)
            .bind(vote.vote_time)
            .bind(vote.superseded)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, vote: Vote) -> Result<Vote> {
        query_as::<_, Vote>(r#"UPDATE "vote" SET "editor" = $2, "edit" = $3, "vote" = $4, "vote_time" = $5, "superseded" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(vote.id)
            .bind(vote.editor)
            .bind(vote.edit)
            .bind(vote.vote)
            .bind(vote.vote_time)
            .bind(vote.superseded)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."vote" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
