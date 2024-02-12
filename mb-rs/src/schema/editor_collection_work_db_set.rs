#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionWork;

pub struct EditorCollectionWorkSet;

impl EditorCollectionWorkSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_work<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, work: i32) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = $1 AND "work" = $2"#)
            .bind(collection)
            .bind(work)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_work_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, work_list: Vec<i32>) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = ANY($1) AND "work" = ANY($2)"#)
            .bind(collection_list)
            .bind(work_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_work_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, work: i32) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = $1 AND "work" = $2"#)
            .bind(collection)
            .bind(work)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_id<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_work: EditorCollectionWork) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"INSERT INTO "editor_collection_work" ("collection", "work", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_work.collection)
            .bind(editor_collection_work.work)
            .bind(editor_collection_work.added)
            .bind(editor_collection_work.position)
            .bind(editor_collection_work.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_work: EditorCollectionWork) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"UPDATE "editor_collection_work" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "collection" = 1 AND "work" = 2 RETURNING *;"#)
            .bind(editor_collection_work.collection)
            .bind(editor_collection_work.work)
            .bind(editor_collection_work.added)
            .bind(editor_collection_work.position)
            .bind(editor_collection_work.comment)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_work" WHERE "collection" = 1 AND "work" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
