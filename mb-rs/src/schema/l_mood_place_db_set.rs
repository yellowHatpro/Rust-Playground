#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::LMoodPlace;

pub struct LMoodPlaceSet;

impl LMoodPlaceSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<LMoodPlace> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_link_id_where_link_is<'e, E: PgExecutor<'e>>(executor: E, link_id: i32) -> Result<Vec<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE link = $1"#)
            .bind(link_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_mood_id_where_entity0_is<'e, E: PgExecutor<'e>>(executor: E, mood_id: i32) -> Result<Vec<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE entity0 = $1"#)
            .bind(mood_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_id_where_entity1_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<LMoodPlace>> {
        query_as::<_, LMoodPlace>(r#"SELECT * FROM "musicbrainz"."l_mood_place" WHERE entity1 = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, l_mood_place: LMoodPlace) -> Result<LMoodPlace> {
        query_as::<_, LMoodPlace>(r#"INSERT INTO "l_mood_place" ("id", "link", "entity0", "entity1", "edits_pending", "last_updated", "link_order", "entity0_credit", "entity1_credit") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *;"#)
            .bind(l_mood_place.last_updated)
            .bind(l_mood_place.entity1)
            .bind(l_mood_place.edits_pending)
            .bind(l_mood_place.entity0_credit)
            .bind(l_mood_place.entity0)
            .bind(l_mood_place.link)
            .bind(l_mood_place.id)
            .bind(l_mood_place.link_order)
            .bind(l_mood_place.entity1_credit)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, l_mood_place: LMoodPlace) -> Result<LMoodPlace> {
        query_as::<_, LMoodPlace>(r#"UPDATE "l_mood_place" SET "link" = $2, "edits_pending" = $5, "last_updated" = $6, "entity0_credit" = $8, "entity1_credit" = $9, "entity0" = $3, "entity1" = $4, "link_order" = $7 WHERE "id" = 1 RETURNING *;"#)
            .bind(l_mood_place.link)
            .bind(l_mood_place.link_order)
            .bind(l_mood_place.entity0)
            .bind(l_mood_place.edits_pending)
            .bind(l_mood_place.entity1)
            .bind(l_mood_place.entity0_credit)
            .bind(l_mood_place.entity1_credit)
            .bind(l_mood_place.last_updated)
            .bind(l_mood_place.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."l_mood_place" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
