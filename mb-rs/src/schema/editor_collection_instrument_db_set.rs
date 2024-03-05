#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionInstrument;

pub struct EditorCollectionInstrumentSet;

impl EditorCollectionInstrumentSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionInstrument>> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_collection_and_instrument<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, instrument: i32) -> Result<EditorCollectionInstrument> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument" WHERE "collection" = $1 AND "instrument" = $2"#)
            .bind(collection)
            .bind(instrument)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_collection_and_instrument_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, instrument_list: Vec<i32>) -> Result<Vec<EditorCollectionInstrument>> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument" WHERE "collection" = ANY($1) AND "instrument" = ANY($2)"#)
            .bind(collection_list)
            .bind(instrument_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_collection_and_instrument_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, instrument: i32) -> Result<Option<EditorCollectionInstrument>> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument" WHERE "collection" = $1 AND "instrument" = $2"#)
            .bind(collection)
            .bind(instrument)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_collection_id_where_collection_is<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionInstrument>> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_instrument_id_where_instrument_is<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<EditorCollectionInstrument>> {
        query_as::<_, EditorCollectionInstrument>(r#"SELECT * FROM "musicbrainz"."editor_collection_instrument" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_instrument: EditorCollectionInstrument) -> Result<EditorCollectionInstrument> {
        query_as::<_, EditorCollectionInstrument>(r#"INSERT INTO "editor_collection_instrument" ("collection", "instrument", "added", "position", "comment") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(editor_collection_instrument.added)
            .bind(editor_collection_instrument.comment)
            .bind(editor_collection_instrument.position)
            .bind(editor_collection_instrument.collection)
            .bind(editor_collection_instrument.instrument)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_instrument: EditorCollectionInstrument) -> Result<EditorCollectionInstrument> {
        query_as::<_, EditorCollectionInstrument>(r#"UPDATE "editor_collection_instrument" SET "position" = $4, "added" = $3, "comment" = $5 WHERE "instrument" = 2 AND "collection" = 1 RETURNING *;"#)
            .bind(editor_collection_instrument.position)
            .bind(editor_collection_instrument.comment)
            .bind(editor_collection_instrument.added)
            .bind(editor_collection_instrument.collection)
            .bind(editor_collection_instrument.instrument)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_instrument" WHERE "instrument" = 2 AND "collection" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
