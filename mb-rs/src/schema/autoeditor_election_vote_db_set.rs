#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AutoeditorElectionVote;

pub struct AutoeditorElectionVoteSet;

impl AutoeditorElectionVoteSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_autoeditor_election_id<'e, E: PgExecutor<'e>>(executor: E, autoeditor_election_id: i32) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE autoeditor_election = $1"#)
            .bind(autoeditor_election_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AutoeditorElectionVote>> {
        query_as::<_, AutoeditorElectionVote>(r#"SELECT * FROM "musicbrainz"."autoeditor_election_vote" WHERE voter = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, autoeditor_election_vote: AutoeditorElectionVote) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"INSERT INTO "autoeditor_election_vote" ("id", "autoeditor_election", "voter", "vote", "vote_time") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(autoeditor_election_vote.id)
            .bind(autoeditor_election_vote.autoeditor_election)
            .bind(autoeditor_election_vote.voter)
            .bind(autoeditor_election_vote.vote)
            .bind(autoeditor_election_vote.vote_time)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, autoeditor_election_vote: AutoeditorElectionVote) -> Result<AutoeditorElectionVote> {
        query_as::<_, AutoeditorElectionVote>(r#"UPDATE "autoeditor_election_vote" SET "autoeditor_election" = $2, "voter" = $3, "vote" = $4, "vote_time" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(autoeditor_election_vote.id)
            .bind(autoeditor_election_vote.autoeditor_election)
            .bind(autoeditor_election_vote.voter)
            .bind(autoeditor_election_vote.vote)
            .bind(autoeditor_election_vote.vote_time)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."autoeditor_election_vote" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
