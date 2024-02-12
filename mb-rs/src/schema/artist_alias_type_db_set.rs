#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistAliasType;

pub struct ArtistAliasTypeSet;

impl ArtistAliasTypeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_alias_type_id<'e, E: PgExecutor<'e>>(executor: E, artist_alias_type_id: i32) -> Result<Vec<ArtistAliasType>> {
        query_as::<_, ArtistAliasType>(r#"SELECT * FROM "musicbrainz"."artist_alias_type" WHERE parent = $1"#)
            .bind(artist_alias_type_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_alias_type: ArtistAliasType) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"INSERT INTO "artist_alias_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(artist_alias_type.id)
            .bind(artist_alias_type.name)
            .bind(artist_alias_type.parent)
            .bind(artist_alias_type.child_order)
            .bind(artist_alias_type.description)
            .bind(artist_alias_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_alias_type: ArtistAliasType) -> Result<ArtistAliasType> {
        query_as::<_, ArtistAliasType>(r#"UPDATE "artist_alias_type" SET "name" = $2, "parent" = $3, "child_order" = $4, "description" = $5, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_alias_type.id)
            .bind(artist_alias_type.name)
            .bind(artist_alias_type.parent)
            .bind(artist_alias_type.child_order)
            .bind(artist_alias_type.description)
            .bind(artist_alias_type.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_alias_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
