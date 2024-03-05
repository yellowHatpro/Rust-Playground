#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AutoeditorElection;

pub struct AutoeditorElectionSet;

impl AutoeditorElectionSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AutoeditorElection> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_candidate_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE candidate = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_proposer_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE proposer = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_seconder_1_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE seconder_1 = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id_where_seconder_2_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<AutoeditorElection>> {
        query_as::<_, AutoeditorElection>(r#"SELECT * FROM "musicbrainz"."autoeditor_election" WHERE seconder_2 = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, autoeditor_election: AutoeditorElection) -> Result<AutoeditorElection> {
        query_as::<_, AutoeditorElection>(r#"INSERT INTO "autoeditor_election" ("id", "candidate", "proposer", "seconder_1", "seconder_2", "status", "yes_votes", "no_votes", "propose_time", "open_time", "close_time") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *;"#)
            .bind(autoeditor_election.id)
            .bind(autoeditor_election.seconder_1)
            .bind(autoeditor_election.open_time)
            .bind(autoeditor_election.close_time)
            .bind(autoeditor_election.seconder_2)
            .bind(autoeditor_election.proposer)
            .bind(autoeditor_election.yes_votes)
            .bind(autoeditor_election.no_votes)
            .bind(autoeditor_election.propose_time)
            .bind(autoeditor_election.candidate)
            .bind(autoeditor_election.status)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, autoeditor_election: AutoeditorElection) -> Result<AutoeditorElection> {
        query_as::<_, AutoeditorElection>(r#"UPDATE "autoeditor_election" SET "open_time" = $10, "close_time" = $11, "no_votes" = $8, "seconder_2" = $5, "status" = $6, "yes_votes" = $7, "proposer" = $3, "propose_time" = $9, "candidate" = $2, "seconder_1" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(autoeditor_election.seconder_1)
            .bind(autoeditor_election.id)
            .bind(autoeditor_election.no_votes)
            .bind(autoeditor_election.propose_time)
            .bind(autoeditor_election.candidate)
            .bind(autoeditor_election.proposer)
            .bind(autoeditor_election.open_time)
            .bind(autoeditor_election.close_time)
            .bind(autoeditor_election.yes_votes)
            .bind(autoeditor_election.seconder_2)
            .bind(autoeditor_election.status)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."autoeditor_election" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
