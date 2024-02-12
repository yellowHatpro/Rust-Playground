#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistAttributeTypeAllowedValue;

pub struct ArtistAttributeTypeAllowedValueSet;

impl ArtistAttributeTypeAllowedValueSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, artist_attribute_type_id: i32) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE artist_attribute_type = $1"#)
            .bind(artist_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, artist_attribute_type_allowed_value_id: i32) -> Result<Vec<ArtistAttributeTypeAllowedValue>> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"SELECT * FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE parent = $1"#)
            .bind(artist_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_attribute_type_allowed_value: ArtistAttributeTypeAllowedValue) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"INSERT INTO "artist_attribute_type_allowed_value" ("id", "artist_attribute_type", "value", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(artist_attribute_type_allowed_value.id)
            .bind(artist_attribute_type_allowed_value.artist_attribute_type)
            .bind(artist_attribute_type_allowed_value.value)
            .bind(artist_attribute_type_allowed_value.parent)
            .bind(artist_attribute_type_allowed_value.child_order)
            .bind(artist_attribute_type_allowed_value.description)
            .bind(artist_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_attribute_type_allowed_value: ArtistAttributeTypeAllowedValue) -> Result<ArtistAttributeTypeAllowedValue> {
        query_as::<_, ArtistAttributeTypeAllowedValue>(r#"UPDATE "artist_attribute_type_allowed_value" SET "artist_attribute_type" = $2, "value" = $3, "parent" = $4, "child_order" = $5, "description" = $6, "gid" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_attribute_type_allowed_value.id)
            .bind(artist_attribute_type_allowed_value.artist_attribute_type)
            .bind(artist_attribute_type_allowed_value.value)
            .bind(artist_attribute_type_allowed_value.parent)
            .bind(artist_attribute_type_allowed_value.child_order)
            .bind(artist_attribute_type_allowed_value.description)
            .bind(artist_attribute_type_allowed_value.gid)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_attribute_type_allowed_value" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
