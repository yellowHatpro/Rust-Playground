#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistAttribute;

pub struct ArtistAttributeSet;

impl ArtistAttributeSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_attribute_type_id<'e, E: PgExecutor<'e>>(executor: E, artist_attribute_type_id: i32) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE artist_attribute_type = $1"#)
            .bind(artist_attribute_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_attribute_type_allowed_value_id<'e, E: PgExecutor<'e>>(executor: E, artist_attribute_type_allowed_value_id: i32) -> Result<Vec<ArtistAttribute>> {
        query_as::<_, ArtistAttribute>(r#"SELECT * FROM "musicbrainz"."artist_attribute" WHERE artist_attribute_type_allowed_value = $1"#)
            .bind(artist_attribute_type_allowed_value_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_attribute: ArtistAttribute) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"INSERT INTO "artist_attribute" ("id", "artist", "artist_attribute_type", "artist_attribute_type_allowed_value", "artist_attribute_text") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(artist_attribute.id)
            .bind(artist_attribute.artist)
            .bind(artist_attribute.artist_attribute_type)
            .bind(artist_attribute.artist_attribute_type_allowed_value)
            .bind(artist_attribute.artist_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_attribute: ArtistAttribute) -> Result<ArtistAttribute> {
        query_as::<_, ArtistAttribute>(r#"UPDATE "artist_attribute" SET "artist" = $2, "artist_attribute_type" = $3, "artist_attribute_type_allowed_value" = $4, "artist_attribute_text" = $5 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_attribute.id)
            .bind(artist_attribute.artist)
            .bind(artist_attribute.artist_attribute_type)
            .bind(artist_attribute.artist_attribute_type_allowed_value)
            .bind(artist_attribute.artist_attribute_text)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_attribute" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
