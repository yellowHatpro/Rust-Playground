#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeSeries;

pub struct EditorSubscribeSeriesSet;

impl EditorSubscribeSeriesSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<EditorSubscribeSeries> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_id_where_series_is<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id_where_last_edit_sent_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditorSubscribeSeries>> {
        query_as::<_, EditorSubscribeSeries>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_series" WHERE last_edit_sent = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_series: EditorSubscribeSeries) -> Result<EditorSubscribeSeries> {
        query_as::<_, EditorSubscribeSeries>(r#"INSERT INTO "editor_subscribe_series" ("id", "editor", "series", "last_edit_sent") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(editor_subscribe_series.id)
            .bind(editor_subscribe_series.last_edit_sent)
            .bind(editor_subscribe_series.editor)
            .bind(editor_subscribe_series.series)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_series: EditorSubscribeSeries) -> Result<EditorSubscribeSeries> {
        query_as::<_, EditorSubscribeSeries>(r#"UPDATE "editor_subscribe_series" SET "editor" = $2, "last_edit_sent" = $4, "series" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(editor_subscribe_series.editor)
            .bind(editor_subscribe_series.series)
            .bind(editor_subscribe_series.id)
            .bind(editor_subscribe_series.last_edit_sent)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_series" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
