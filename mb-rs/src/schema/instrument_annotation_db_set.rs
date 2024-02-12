#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::InstrumentAnnotation;

pub struct InstrumentAnnotationSet;

impl InstrumentAnnotationSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_instrument_and_annotation<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, annotation: i32) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "instrument" = $1 AND "annotation" = $2"#)
            .bind(instrument)
            .bind(annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_instrument_and_annotation_list<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_list: Vec<i32>, annotation_list: Vec<i32>) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "instrument" = ANY($1) AND "annotation" = ANY($2)"#)
            .bind(instrument_list)
            .bind(annotation_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_instrument_and_annotation_optional<'e, E: PgExecutor<'e>>(&self, executor: E, instrument: i32, annotation: i32) -> Result<Option<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "instrument" = $1 AND "annotation" = $2"#)
            .bind(instrument)
            .bind(annotation)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_instrument_id<'e, E: PgExecutor<'e>>(executor: E, instrument_id: i32) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE instrument = $1"#)
            .bind(instrument_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_annotation_id<'e, E: PgExecutor<'e>>(executor: E, annotation_id: i32) -> Result<Vec<InstrumentAnnotation>> {
        query_as::<_, InstrumentAnnotation>(r#"SELECT * FROM "musicbrainz"."instrument_annotation" WHERE annotation = $1"#)
            .bind(annotation_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_annotation: InstrumentAnnotation) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"INSERT INTO "instrument_annotation" ("instrument", "annotation") VALUES ($1, $2) RETURNING *;"#)
            .bind(instrument_annotation.instrument)
            .bind(instrument_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, instrument_annotation: InstrumentAnnotation) -> Result<InstrumentAnnotation> {
        query_as::<_, InstrumentAnnotation>(r#"UPDATE "instrument_annotation" SET  WHERE "instrument" = 1 AND "annotation" = 2 RETURNING *;"#)
            .bind(instrument_annotation.instrument)
            .bind(instrument_annotation.annotation)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."instrument_annotation" WHERE "instrument" = 1 AND "annotation" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
