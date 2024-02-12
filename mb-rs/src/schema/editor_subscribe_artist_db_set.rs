#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeArtist;

pub struct EditorSubscribeArtistSet;

impl EditorSubscribeArtistSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditorSubscribeArtist>> {
        query_as::<_, EditorSubscribeArtist>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist" WHERE last_edit_sent = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_artist: EditorSubscribeArtist) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"INSERT INTO "editor_subscribe_artist" ("id", "editor", "artist", "last_edit_sent") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(editor_subscribe_artist.id)
            .bind(editor_subscribe_artist.editor)
            .bind(editor_subscribe_artist.artist)
            .bind(editor_subscribe_artist.last_edit_sent)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_artist: EditorSubscribeArtist) -> Result<EditorSubscribeArtist> {
        query_as::<_, EditorSubscribeArtist>(r#"UPDATE "editor_subscribe_artist" SET "editor" = $2, "artist" = $3, "last_edit_sent" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_subscribe_artist.id)
            .bind(editor_subscribe_artist.editor)
            .bind(editor_subscribe_artist.artist)
            .bind(editor_subscribe_artist.last_edit_sent)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_artist" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
