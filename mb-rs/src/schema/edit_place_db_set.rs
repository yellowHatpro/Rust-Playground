#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditPlace;

pub struct EditPlaceSet;

impl EditPlaceSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_place<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, place: i32) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = $1 AND "place" = $2"#)
            .bind(edit)
            .bind(place)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_place_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, place_list: Vec<i32>) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = ANY($1) AND "place" = ANY($2)"#)
            .bind(edit_list)
            .bind(place_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_place_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, place: i32) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = $1 AND "place" = $2"#)
            .bind(edit)
            .bind(place)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_id<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_place: EditPlace) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"INSERT INTO "edit_place" ("edit", "place") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_place.edit)
            .bind(edit_place.place)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_place: EditPlace) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"UPDATE "edit_place" SET  WHERE "edit" = 1 AND "place" = 2 RETURNING *;"#)
            .bind(edit_place.edit)
            .bind(edit_place.place)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_place" WHERE "edit" = 1 AND "place" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
