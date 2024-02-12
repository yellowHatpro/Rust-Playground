#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ReleasePackaging;

pub struct ReleasePackagingSet;

impl ReleasePackagingSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_release_packaging_id<'e, E: PgExecutor<'e>>(executor: E, release_packaging_id: i32) -> Result<Vec<ReleasePackaging>> {
        query_as::<_, ReleasePackaging>(r#"SELECT * FROM "musicbrainz"."release_packaging" WHERE parent = $1"#)
            .bind(release_packaging_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, release_packaging: ReleasePackaging) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"INSERT INTO "release_packaging" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(release_packaging.id)
            .bind(release_packaging.name)
            .bind(release_packaging.parent)
            .bind(release_packaging.child_order)
            .bind(release_packaging.description)
            .bind(release_packaging.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, release_packaging: ReleasePackaging) -> Result<ReleasePackaging> {
        query_as::<_, ReleasePackaging>(r#"UPDATE "release_packaging" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(release_packaging.id)
            .bind(release_packaging.name)
            .bind(release_packaging.parent)
            .bind(release_packaging.child_order)
            .bind(release_packaging.description)
            .bind(release_packaging.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."release_packaging" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
