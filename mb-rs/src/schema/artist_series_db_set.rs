#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistSeries;

pub struct ArtistSeriesSet;

impl ArtistSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements

// SELECT many by Primary Key statements

// SELECT by Primary Key statements
    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<ArtistSeries>> {
        query_as::<_, ArtistSeries>(r#"SELECT * FROM "musicbrainz"."artist_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_series: ArtistSeries) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"INSERT INTO "artist_series" ("artist", "series", "relationship", "link_order", "link", "text_value") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(artist_series.series)
            .bind(artist_series.link_order)
            .bind(artist_series.artist)
            .bind(artist_series.link)
            .bind(artist_series.text_value)
            .bind(artist_series.relationship)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_series: ArtistSeries) -> Result<ArtistSeries> {
        query_as::<_, ArtistSeries>(r#"UPDATE "artist_series" SET "series" = $2, "artist" = $1, "link_order" = $4, "link" = $5, "text_value" = $6, "relationship" = $3 WHERE  RETURNING *;"#)
            .bind(artist_series.artist)
            .bind(artist_series.relationship)
            .bind(artist_series.series)
            .bind(artist_series.link_order)
            .bind(artist_series.text_value)
            .bind(artist_series.link)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_series" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
