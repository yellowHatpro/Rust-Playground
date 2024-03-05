#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LRecordingRelease;

pub struct LRecordingReleaseSet;

impl LRecordingReleaseSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LRecordingRelease> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_recording_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, recording_id: i32) -> Result<Vec<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE entity0 = $1"#)
            .bind(recording_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_release_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, release_id: i32) -> Result<Vec<LRecordingRelease>> {
        query_as::<_, LRecordingRelease>(r#"SELECT * FROM "musicbrainz"."l_recording_release" WHERE entity1 = $1"#)
            .bind(release_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_recording_release: LRecordingRelease) -> Result<LRecordingRelease> {
        query_as::<_, LRecordingRelease>(r#"INSERT INTO "l_recording_release" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_recording_release.last_updated)
            .bind(l_recording_release.entity1_credit)
            .bind(l_recording_release.edits_pending)
            .bind(l_recording_release.link_order)
            .bind(l_recording_release.entity1)
            .bind(l_recording_release.id)
            .bind(l_recording_release.link)
            .bind(l_recording_release.entity0_credit)
            .bind(l_recording_release.entity0)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_recording_release: LRecordingRelease) -> Result<LRecordingRelease> {
        query_as::<_, LRecordingRelease>(r#"UPDATE "l_recording_release" SET "last_updated" = $6, "entity0" = $3, "entity1" = $4, "link_order" = $7, "edits_pending" = $5, "entity1_credit" = $9, "link" = $2, "entity0_credit" = $8 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_recording_release.edits_pending)
            .bind(l_recording_release.last_updated)
            .bind(l_recording_release.entity1_credit)
            .bind(l_recording_release.id)
            .bind(l_recording_release.entity0)
            .bind(l_recording_release.link_order)
            .bind(l_recording_release.link)
            .bind(l_recording_release.entity1)
            .bind(l_recording_release.entity0_credit)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_recording_release" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
