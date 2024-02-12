#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Release;

pub struct ReleaseSet;

impl ReleaseSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Release> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Release> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Release> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Release> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Release> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_credit_id<'e, E: PgExecutor<'e>>(executor: E, artist_credit_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE artist_credit = $1"#)
            .bind(artist_credit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_status_id<'e, E: PgExecutor<'e>>(executor: E, release_status_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE status = $1"#)
            .bind(release_status_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_packaging_id<'e, E: PgExecutor<'e>>(executor: E, release_packaging_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE packaging = $1"#)
            .bind(release_packaging_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_language_id<'e, E: PgExecutor<'e>>(executor: E, language_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE language = $1"#)
            .bind(language_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_script_id<'e, E: PgExecutor<'e>>(executor: E, script_id: i32) -> Result<Vec<Release>> {
        query_as::<_, Release>(r#"SELECT * FROM "musicbrainz"."release" WHERE script = $1"#)
            .bind(script_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release: Release) -> Result<Release> {
        query_as::<_, Release>(r#"INSERT INTO "release" ("id", "gid", "name", "artist_credit", "release_group", "status", "packaging", "language", "script", "barcode", "comment", "edits_pending", "quality", "last_updated") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING *;"#)
            .bind(release.id)
            .bind(release.gid)
            .bind(release.name)
            .bind(release.artist_credit)
            .bind(release.release_group)
            .bind(release.status)
            .bind(release.packaging)
            .bind(release.language)
            .bind(release.script)
            .bind(release.barcode)
            .bind(release.comment)
            .bind(release.edits_pending)
            .bind(release.quality)
            .bind(release.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release: Release) -> Result<Release> {
        query_as::<_, Release>(r#"UPDATE "release" SET "gid" = $2, "name" = $3, "artist_credit" = $4, "release_group" = $5, "status" = $6, "packaging" = $7, "language" = $8, "script" = $9, "barcode" = $10, "comment" = $11, "edits_pending" = $12, "quality" = $13, "last_updated" = $14 WHERE "id" = 1 RETURNING *;"#)
            .bind(release.id)
            .bind(release.gid)
            .bind(release.name)
            .bind(release.artist_credit)
            .bind(release.release_group)
            .bind(release.status)
            .bind(release.packaging)
            .bind(release.language)
            .bind(release.script)
            .bind(release.barcode)
            .bind(release.comment)
            .bind(release.edits_pending)
            .bind(release.quality)
            .bind(release.last_updated)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
