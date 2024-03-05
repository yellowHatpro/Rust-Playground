#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistCredit;

pub struct ArtistCreditSet;

impl ArtistCreditSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistCredit>> {
        query_as::<_, ArtistCredit>(r#"SELECT * FROM "musicbrainz"."artist_credit""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistCredit> {
        query_as::<_, ArtistCredit>(r#"SELECT * FROM "musicbrainz"."artist_credit" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistCredit>> {
        query_as::<_, ArtistCredit>(r#"SELECT * FROM "musicbrainz"."artist_credit" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistCredit>> {
        query_as::<_, ArtistCredit>(r#"SELECT * FROM "musicbrainz"."artist_credit" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: ArtistCredit) -> Result<ArtistCredit> {
        query_as::<_, ArtistCredit>(r#"INSERT INTO "artist_credit" ("id", "name", "artist_count", "ref_count", "created", "edits_pending", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(artist_credit.id)
            .bind(artist_credit.artist_count)
            .bind(artist_credit.created)
            .bind(artist_credit.name)
            .bind(artist_credit.gid)
            .bind(artist_credit.ref_count)
            .bind(artist_credit.edits_pending)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_credit: ArtistCredit) -> Result<ArtistCredit> {
        query_as::<_, ArtistCredit>(r#"UPDATE "artist_credit" SET "name" = $2, "gid" = $7, "ref_count" = $4, "edits_pending" = $6, "artist_count" = $3, "created" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_credit.artist_count)
            .bind(artist_credit.gid)
            .bind(artist_credit.id)
            .bind(artist_credit.created)
            .bind(artist_credit.ref_count)
            .bind(artist_credit.name)
            .bind(artist_credit.edits_pending)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_credit" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
