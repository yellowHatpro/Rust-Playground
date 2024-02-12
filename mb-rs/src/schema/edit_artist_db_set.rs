#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditArtist;

pub struct EditArtistSet;

impl EditArtistSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_artist<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, artist: i32) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "edit" = $1 AND "artist" = $2"#)
            .bind(edit)
            .bind(artist)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_artist_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, artist_list: Vec<i32>) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "edit" = ANY($1) AND "artist" = ANY($2)"#)
            .bind(edit_list)
            .bind(artist_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_artist_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, artist: i32) -> Result<Option<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "edit" = $1 AND "artist" = $2"#)
            .bind(edit)
            .bind(artist)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<EditArtist>> {
        query_as::<_, EditArtist>(r#"SELECT * FROM "musicbrainz"."edit_artist" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_artist: EditArtist) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"INSERT INTO "edit_artist" ("edit", "artist", "status") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(edit_artist.edit)
            .bind(edit_artist.artist)
            .bind(edit_artist.status)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_artist: EditArtist) -> Result<EditArtist> {
        query_as::<_, EditArtist>(r#"UPDATE "edit_artist" SET "status" = $3 WHERE "edit" = 1 AND "artist" = 2 RETURNING *;"#)
            .bind(edit_artist.edit)
            .bind(edit_artist.artist)
            .bind(edit_artist.status)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_artist" WHERE "edit" = 1 AND "artist" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
