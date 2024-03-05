#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionLabel;

pub struct EditorCollectionLabelSet;

impl EditorCollectionLabelSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionLabel>> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_label<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, label: i32) -> Result<EditorCollectionLabel> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label" WHERE "collection" = $1 AND "label" = $2"#)
            .bind(collection)
            .bind(label)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_label_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, label_list: Vec<i32>) -> Result<Vec<EditorCollectionLabel>> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label" WHERE "collection" = ANY($1) AND "label" = ANY($2)"#)
            .bind(collection_list)
            .bind(label_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_label_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, label: i32) -> Result<Option<EditorCollectionLabel>> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label" WHERE "collection" = $1 AND "label" = $2"#)
            .bind(collection)
            .bind(label)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionLabel>> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_label_id_where_label_is<'e, E: PgExecutor<'e>>(executor: E, label_id: i32) -> Result<Vec<EditorCollectionLabel>> {
        query_as::<_, EditorCollectionLabel>(r#"SELECT * FROM "musicbrainz"."editor_collection_label" WHERE label = $1"#)
            .bind(label_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_label: EditorCollectionLabel) -> Result<EditorCollectionLabel> {
        query_as::<_, EditorCollectionLabel>(r#"INSERT INTO "editor_collection_label" ("collection", "label", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_label.collection)
            .bind(editor_collection_label.added)
            .bind(editor_collection_label.label)
            .bind(editor_collection_label.comment)
            .bind(editor_collection_label.position)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_label: EditorCollectionLabel) -> Result<EditorCollectionLabel> {
        query_as::<_, EditorCollectionLabel>(r#"UPDATE "editor_collection_label" SET "position" = $4, "added" = $3, "comment" = $5 WHERE "label" = 2 AND "collection" = 1 RETURNING *;"#)
            .bind(editor_collection_label.comment)
            .bind(editor_collection_label.added)
            .bind(editor_collection_label.collection)
            .bind(editor_collection_label.label)
            .bind(editor_collection_label.position)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_label" WHERE "collection" = 1 AND "label" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
