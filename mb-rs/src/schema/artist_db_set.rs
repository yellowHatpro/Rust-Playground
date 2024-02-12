#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Artist;

pub struct ArtistSet;

impl ArtistSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_artist_type_id<'e, E: PgExecutor<'e>>(executor: E, artist_type_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE type = $1"#)
            .bind(artist_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_gender_id<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE gender = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE begin_area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE end_area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist: Artist) -> Result<Artist> {
        query_as::<_, Artist>(r#"INSERT INTO "artist" ("id", "gid", "name", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "type", "area", "gender", "comment", "edits_pending", "last_updated", "ended", "begin_area", "end_area") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) RETURNING *;"#)
            .bind(artist.id)
            .bind(artist.gid)
            .bind(artist.name)
            .bind(artist.sort_name)
            .bind(artist.begin_date_year)
            .bind(artist.begin_date_month)
            .bind(artist.begin_date_day)
            .bind(artist.end_date_year)
            .bind(artist.end_date_month)
            .bind(artist.end_date_day)
            .bind(artist.Type)
            .bind(artist.area)
            .bind(artist.gender)
            .bind(artist.comment)
            .bind(artist.edits_pending)
            .bind(artist.last_updated)
            .bind(artist.ended)
            .bind(artist.begin_area)
            .bind(artist.end_area)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist: Artist) -> Result<Artist> {
        query_as::<_, Artist>(r#"UPDATE "artist" SET "gid" = $2, "name" = $3, "sort_name" = $4, "begin_date_year" = $5, "begin_date_month" = $6, "begin_date_day" = $7, "end_date_year" = $8, "end_date_month" = $9, "end_date_day" = $10, "type" = $11, "area" = $12, "gender" = $13, "comment" = $14, "edits_pending" = $15, "last_updated" = $16, "ended" = $17, "begin_area" = $18, "end_area" = $19 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist.id)
            .bind(artist.gid)
            .bind(artist.name)
            .bind(artist.sort_name)
            .bind(artist.begin_date_year)
            .bind(artist.begin_date_month)
            .bind(artist.begin_date_day)
            .bind(artist.end_date_year)
            .bind(artist.end_date_month)
            .bind(artist.end_date_day)
            .bind(artist.Type)
            .bind(artist.area)
            .bind(artist.gender)
            .bind(artist.comment)
            .bind(artist.edits_pending)
            .bind(artist.last_updated)
            .bind(artist.ended)
            .bind(artist.begin_area)
            .bind(artist.end_area)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
