#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditEvent;

pub struct EditEventSet;

impl EditEventSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_event<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, event: i32) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = $1 AND "event" = $2"#)
            .bind(edit)
            .bind(event)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_event_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, event_list: Vec<i32>) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = ANY($1) AND "event" = ANY($2)"#)
            .bind(edit_list)
            .bind(event_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_event_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, event: i32) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "edit" = $1 AND "event" = $2"#)
            .bind(edit)
            .bind(event)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_event_id<'e, E: PgExecutor<'e>>(executor: E, event_id: i32) -> Result<Vec<EditEvent>> {
        query_as::<_, EditEvent>(r#"SELECT * FROM "musicbrainz"."edit_event" WHERE event = $1"#)
            .bind(event_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_event: EditEvent) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"INSERT INTO "edit_event" ("edit", "event") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_event.edit)
            .bind(edit_event.event)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_event: EditEvent) -> Result<EditEvent> {
        query_as::<_, EditEvent>(r#"UPDATE "edit_event" SET  WHERE "edit" = 1 AND "event" = 2 RETURNING *;"#)
            .bind(edit_event.edit)
            .bind(edit_event.event)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_event" WHERE "edit" = 1 AND "event" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
