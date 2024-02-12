#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditSeries;

pub struct EditSeriesSet;

impl EditSeriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_series<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, series: i32) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "edit" = $1 AND "series" = $2"#)
            .bind(edit)
            .bind(series)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_series_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, series_list: Vec<i32>) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "edit" = ANY($1) AND "series" = ANY($2)"#)
            .bind(edit_list)
            .bind(series_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_series_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, series: i32) -> Result<Option<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "edit" = $1 AND "series" = $2"#)
            .bind(edit)
            .bind(series)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_series_id<'e, E: PgExecutor<'e>>(executor: E, series_id: i32) -> Result<Vec<EditSeries>> {
        query_as::<_, EditSeries>(r#"SELECT * FROM "musicbrainz"."edit_series" WHERE series = $1"#)
            .bind(series_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_series: EditSeries) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"INSERT INTO "edit_series" ("edit", "series") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_series.edit)
            .bind(edit_series.series)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_series: EditSeries) -> Result<EditSeries> {
        query_as::<_, EditSeries>(r#"UPDATE "edit_series" SET  WHERE "edit" = 1 AND "series" = 2 RETURNING *;"#)
            .bind(edit_series.edit)
            .bind(edit_series.series)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_series" WHERE "edit" = 1 AND "series" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
