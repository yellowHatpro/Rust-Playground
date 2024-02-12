#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionArtist;

pub struct EditorCollectionArtistSet;

impl EditorCollectionArtistSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_artist<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, artist: i32) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "collection" = $1 AND "artist" = $2"#)
            .bind(collection)
            .bind(artist)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_artist_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, artist_list: Vec<i32>) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "collection" = ANY($1) AND "artist" = ANY($2)"#)
            .bind(collection_list)
            .bind(artist_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_artist_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, artist: i32) -> Result<Option<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "collection" = $1 AND "artist" = $2"#)
            .bind(collection)
            .bind(artist)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_artist_id<'e, E: PgExecutor<'e>>(executor: E, artist_id: i32) -> Result<Vec<EditorCollectionArtist>> {
        query_as::<_, EditorCollectionArtist>(r#"SELECT * FROM "musicbrainz"."editor_collection_artist" WHERE artist = $1"#)
            .bind(artist_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_artist: EditorCollectionArtist) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"INSERT INTO "editor_collection_artist" ("collection", "artist", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_artist.collection)
            .bind(editor_collection_artist.artist)
            .bind(editor_collection_artist.added)
            .bind(editor_collection_artist.position)
            .bind(editor_collection_artist.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_artist: EditorCollectionArtist) -> Result<EditorCollectionArtist> {
        query_as::<_, EditorCollectionArtist>(r#"UPDATE "editor_collection_artist" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "artist" = 2 RETURNING *;"#)
            .bind(editor_collection_artist.collection)
            .bind(editor_collection_artist.artist)
            .bind(editor_collection_artist.added)
            .bind(editor_collection_artist.position)
            .bind(editor_collection_artist.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_artist" WHERE "collection" = 1 AND "artist" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
