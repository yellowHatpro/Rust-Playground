#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionReleaseGroup;

pub struct EditorCollectionReleaseGroupSet;

impl EditorCollectionReleaseGroupSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_release_group<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release_group: i32) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = $1 AND "release_group" = $2"#)
            .bind(collection)
            .bind(release_group)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_release_group_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, release_group_list: Vec<i32>) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = ANY($1) AND "release_group" = ANY($2)"#)
            .bind(collection_list)
            .bind(release_group_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_release_group_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, release_group: i32) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = $1 AND "release_group" = $2"#)
            .bind(collection)
            .bind(release_group)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_group_id<'e, E: PgExecutor<'e>>(executor: E, release_group_id: i32) -> Result<Vec<EditorCollectionReleaseGroup>> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"SELECT * FROM "musicbrainz"."editor_collection_release_group" WHERE release_group = $1"#)
            .bind(release_group_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release_group: EditorCollectionReleaseGroup) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"INSERT INTO "editor_collection_release_group" ("collection", "release_group", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_release_group.collection)
            .bind(editor_collection_release_group.release_group)
            .bind(editor_collection_release_group.added)
            .bind(editor_collection_release_group.position)
            .bind(editor_collection_release_group.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_release_group: EditorCollectionReleaseGroup) -> Result<EditorCollectionReleaseGroup> {
        query_as::<_, EditorCollectionReleaseGroup>(r#"UPDATE "editor_collection_release_group" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "release_group" = 2 RETURNING *;"#)
            .bind(editor_collection_release_group.collection)
            .bind(editor_collection_release_group.release_group)
            .bind(editor_collection_release_group.added)
            .bind(editor_collection_release_group.position)
            .bind(editor_collection_release_group.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_release_group" WHERE "collection" = 1 AND "release_group" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
