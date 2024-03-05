#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Artist;

pub struct ArtistSet;

impl ArtistSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Artist> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_type_id_where_Type_is<'e, E: PgExecutor<'e>>(executor: E, artist_type_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE type = $1"#)
            .bind(artist_type_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_gender_id_where_gender_is<'e, E: PgExecutor<'e>>(executor: E, gender_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE gender = $1"#)
            .bind(gender_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_begin_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE begin_area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id_where_end_area_is<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<Artist>> {
        query_as::<_, Artist>(r#"SELECT * FROM "musicbrainz"."artist" WHERE end_area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist: Artist) -> Result<Artist> {
        query_as::<_, Artist>(r#"INSERT INTO "artist" ("id", "gid", "name", "sort_name", "begin_date_year", "begin_date_month", "begin_date_day", "end_date_year", "end_date_month", "end_date_day", "type", "area", "gender", "comment", "edits_pending", "last_updated", "ended", "begin_area", "end_area") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) RETURNING *;"#)
            .bind(artist.id)
            .bind(artist.name)
            .bind(artist.gender)
            .bind(artist.ended)
            .bind(artist.sort_name)
            .bind(artist.last_updated)
            .bind(artist.end_date_day)
            .bind(artist.end_date_year)
            .bind(artist.area)
            .bind(artist.end_area)
            .bind(artist.comment)
            .bind(artist.gid)
            .bind(artist.begin_date_day)
            .bind(artist.end_date_month)
            .bind(artist.begin_area)
            .bind(artist.Type)
            .bind(artist.begin_date_year)
            .bind(artist.edits_pending)
            .bind(artist.begin_date_month)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist: Artist) -> Result<Artist> {
        query_as::<_, Artist>(r#"UPDATE "artist" SET "area" = $12, "sort_name" = $4, "gid" = $2, "ended" = $17, "end_date_year" = $8, "end_date_day" = $10, "edits_pending" = $15, "end_date_month" = $9, "type" = $11, "begin_date_year" = $5, "begin_date_day" = $7, "begin_date_month" = $6, "begin_area" = $18, "end_area" = $19, "gender" = $13, "name" = $3, "last_updated" = $16, "comment" = $14 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist.end_date_year)
            .bind(artist.gender)
            .bind(artist.end_date_day)
            .bind(artist.begin_date_day)
            .bind(artist.area)
            .bind(artist.comment)
            .bind(artist.edits_pending)
            .bind(artist.begin_area)
            .bind(artist.id)
            .bind(artist.Type)
            .bind(artist.sort_name)
            .bind(artist.gid)
            .bind(artist.begin_date_year)
            .bind(artist.end_date_month)
            .bind(artist.end_area)
            .bind(artist.begin_date_month)
            .bind(artist.last_updated)
            .bind(artist.name)
            .bind(artist.ended)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
