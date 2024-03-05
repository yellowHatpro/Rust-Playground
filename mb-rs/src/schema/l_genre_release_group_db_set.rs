#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LGenreReleaseGroup;

pub struct LGenreReleaseGroupSet;

impl LGenreReleaseGroupSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LGenreReleaseGroup> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_genre_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, genre_id: i32) -> Result<Vec<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE entity0 = $1"#)
            .bind(genre_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<LGenreReleaseGroup>> {
        query_as::<_, LGenreReleaseGroup>(r#"SELECT * FROM "musicbrainz"."l_genre_release_group" WHERE entity1 = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_release_group: LGenreReleaseGroup) -> Result<LGenreReleaseGroup> {
        query_as::<_, LGenreReleaseGroup>(r#"INSERT INTO "l_genre_release_group" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_genre_release_group.edits_pending)
            .bind(l_genre_release_group.entity0_credit)
            .bind(l_genre_release_group.link)
            .bind(l_genre_release_group.link_order)
            .bind(l_genre_release_group.entity0)
            .bind(l_genre_release_group.entity1)
            .bind(l_genre_release_group.id)
            .bind(l_genre_release_group.last_updated)
            .bind(l_genre_release_group.entity1_credit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_genre_release_group: LGenreReleaseGroup) -> Result<LGenreReleaseGroup> {
        query_as::<_, LGenreReleaseGroup>(r#"UPDATE "l_genre_release_group" SET "link_order" = $7, "link" = $2, "edits_pending" = $5, "entity0" = $3, "last_updated" = $6, "entity0_credit" = $8, "entity1_credit" = $9, "entity1" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_genre_release_group.entity1)
            .bind(l_genre_release_group.link)
            .bind(l_genre_release_group.entity0)
            .bind(l_genre_release_group.edits_pending)
            .bind(l_genre_release_group.link_order)
            .bind(l_genre_release_group.entity0_credit)
            .bind(l_genre_release_group.last_updated)
            .bind(l_genre_release_group.entity1_credit)
            .bind(l_genre_release_group.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_genre_release_group" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
