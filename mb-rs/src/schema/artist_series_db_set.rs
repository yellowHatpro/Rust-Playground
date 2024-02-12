#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistSeries;

pub struct ArtistSeriesSet;

impl ArtistSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_series: ArtistSeries) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"INSERT INTO "artist_series" ("artist", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(artist_series.artist)
            .bind(artist_series.series)
            .bind(artist_series.relationship)
            .bind(artist_series.link_order)
            .bind(artist_series.link)
            .bind(artist_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_series: ArtistSeries) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"UPDATE "artist_series" SET "artist" = $1, "series" = $2, "relationship" = $3, "link_order" = $4, "link" = $5, "text_value" = $6 WHERE  RETURNING *;"#)
            .bind(artist_series.artist)
            .bind(artist_series.series)
            .bind(artist_series.relationship)
            .bind(artist_series.link_order)
            .bind(artist_series.link)
            .bind(artist_series.text_value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
