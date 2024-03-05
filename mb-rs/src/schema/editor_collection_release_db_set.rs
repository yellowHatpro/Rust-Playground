#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionRelease;

pub struct EditorCollectionReleaseSet;

impl EditorCollectionReleaseSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionRelease>> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_release<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release: i32) -> Result<EditorCollectionRelease> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release" WHERE "collection" = $1 AND "release" = $2"#)
            .bind(collection)
            .bind(release)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, release_list: Vec<i32>) -> Result<Vec<EditorCollectionRelease>> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release" WHERE "collection" = ANY($1) AND "release" = ANY($2)"#)
            .bind(collection_list)
            .bind(release_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release: i32) -> Result<Option<EditorCollectionRelease>> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release" WHERE "collection" = $1 AND "release" = $2"#)
            .bind(collection)
            .bind(release)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionRelease>> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id_where_release_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<EditorCollectionRelease>> {
        query_as::<_, EditorCollectionRelease>(r#"SELECT * FROM "musicbrainz"."editor_collection_release" WHERE release = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release: EditorCollectionRelease) -> Result<EditorCollectionRelease> {
        query_as::<_, EditorCollectionRelease>(r#"INSERT INTO "editor_collection_release" ("collection", "release", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_release.collection)
            .bind(editor_collection_release.release)
            .bind(editor_collection_release.position)
            .bind(editor_collection_release.comment)
            .bind(editor_collection_release.added)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release: EditorCollectionRelease) -> Result<EditorCollectionRelease> {
        query_as::<_, EditorCollectionRelease>(r#"UPDATE "editor_collection_release" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "release" = 2 RETURNING *;"#)
            .bind(editor_collection_release.collection)
            .bind(editor_collection_release.release)
            .bind(editor_collection_release.added)
            .bind(editor_collection_release.position)
            .bind(editor_collection_release.comment)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_release" WHERE "collection" = 1 AND "release" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
