#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionReleaseGroup;

pub struct EditorCollectionReleaseGroupSet;

impl EditorCollectionReleaseGroupSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_release_group<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release_group: i32) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = $1 AND "release_group" = $2"#)
            .bind(collection)
            .bind(release_group)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_release_group_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, release_group_list: Vec<i32>) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = ANY($1) AND "release_group" = ANY($2)"#)
            .bind(collection_list)
            .bind(release_group_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_release_group_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release_group: i32) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = $1 AND "release_group" = $2"#)
            .bind(collection)
            .bind(release_group)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id_where_release_group_is<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release_group: EditorCollectionReleaseGroup) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"INSERT INTO "editor_collection_release_group" ("collection", "release_group", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_release_group.release_group)
            .bind(editor_collection_release_group.added)
            .bind(editor_collection_release_group.position)
            .bind(editor_collection_release_group.comment)
            .bind(editor_collection_release_group.collection)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release_group: EditorCollectionReleaseGroup) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"UPDATE "editor_collection_release_group" SET "comment" = $5, "added" = $3, "position" = $4 WHERE "collection" = 1 AND "release_group" = 2 RETURNING *;"#)
            .bind(editor_collection_release_group.added)
            .bind(editor_collection_release_group.release_group)
            .bind(editor_collection_release_group.position)
            .bind(editor_collection_release_group.collection)
            .bind(editor_collection_release_group.comment)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_release_group" WHERE "release_group" = 2 AND "collection" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
