#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditPlace;

pub struct EditPlaceSet;

impl EditPlaceSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_edit_and_place<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, place: i32) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = $1 AND "place" = $2"#)
            .bind(edit)
            .bind(place)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_edit_and_place_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, place_list: Vec<i32>) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = ANY($1) AND "place" = ANY($2)"#)
            .bind(edit_list)
            .bind(place_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_edit_and_place_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, place: i32) -> Result<Option<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE "edit" = $1 AND "place" = $2"#)
            .bind(edit)
            .bind(place)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_edit_id_where_edit_is<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_place_id_where_place_is<'e, E: PgExecutor<'e>>(executor: E, place_id: i32) -> Result<Vec<EditPlace>> {
        query_as::<_, EditPlace>(r#"SELECT * FROM "musicbrainz"."edit_place" WHERE place = $1"#)
            .bind(place_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_place: EditPlace) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"INSERT INTO "edit_place" ("edit", "place") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_place.edit)
            .bind(edit_place.place)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_place: EditPlace) -> Result<EditPlace> {
        query_as::<_, EditPlace>(r#"UPDATE "edit_place" SET  WHERE "place" = 2 AND "edit" = 1 RETURNING *;"#)
            .bind(edit_place.edit)
            .bind(edit_place.place)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_place" WHERE "place" = 2 AND "edit" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
