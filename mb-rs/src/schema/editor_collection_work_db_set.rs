#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionWork;

pub struct EditorCollectionWorkSet;

impl EditorCollectionWorkSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_work<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, work: i32) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = $1 AND "work" = $2"#)
            .bind(collection)
            .bind(work)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_work_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, work_list: Vec<i32>) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = ANY($1) AND "work" = ANY($2)"#)
            .bind(collection_list)
            .bind(work_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_work_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, work: i32) -> Result<Option<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE "collection" = $1 AND "work" = $2"#)
            .bind(collection)
            .bind(work)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_work_id_where_work_is<'e, E: PgExecutor<'e>>(executor: E, work_id: i32) -> Result<Vec<EditorCollectionWork>> {
        query_as::<_, EditorCollectionWork>(r#"SELECT * FROM "musicbrainz"."editor_collection_work" WHERE work = $1"#)
            .bind(work_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_work: EditorCollectionWork) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"INSERT INTO "editor_collection_work" ("collection", "work", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_work.work)
            .bind(editor_collection_work.collection)
            .bind(editor_collection_work.comment)
            .bind(editor_collection_work.position)
            .bind(editor_collection_work.added)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_work: EditorCollectionWork) -> Result<EditorCollectionWork> {
        query_as::<_, EditorCollectionWork>(r#"UPDATE "editor_collection_work" SET "added" = $3, "position" = $4, "comment" = $5 WHERE "work" = 2 AND "collection" = 1 RETURNING *;"#)
            .bind(editor_collection_work.work)
            .bind(editor_collection_work.comment)
            .bind(editor_collection_work.position)
            .bind(editor_collection_work.collection)
            .bind(editor_collection_work.added)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_work" WHERE "collection" = 1 AND "work" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
