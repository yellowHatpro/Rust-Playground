#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::MediumTrackDurations;

pub struct MediumTrackDurationsSet;

impl MediumTrackDurationsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations""#)
            .fetch_all(executor)
            .await
    }



    pub async fn by__optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<MediumTrackDurations>> {
        query_as::<_, MediumTrackDurations>(r#"SELECT * FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, medium_track_durations: MediumTrackDurations) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"INSERT INTO "medium_track_durations" ("medium", "pregap_length", "cdtoc_track_lengths", "data_track_lengths") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(medium_track_durations.medium)
            .bind(medium_track_durations.pregap_length)
            .bind(medium_track_durations.cdtoc_track_lengths)
            .bind(medium_track_durations.data_track_lengths)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, medium_track_durations: MediumTrackDurations) -> Result<MediumTrackDurations> {
        query_as::<_, MediumTrackDurations>(r#"UPDATE "medium_track_durations" SET "medium" = $1, "pregap_length" = $2, "cdtoc_track_lengths" = $3, "data_track_lengths" = $4 WHERE  RETURNING *;"#)
            .bind(medium_track_durations.medium)
            .bind(medium_track_durations.pregap_length)
            .bind(medium_track_durations.cdtoc_track_lengths)
            .bind(medium_track_durations.data_track_lengths)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."medium_track_durations" WHERE "#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
