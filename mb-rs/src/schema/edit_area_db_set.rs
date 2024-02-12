#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditArea;

pub struct EditAreaSet;

impl EditAreaSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_area<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, area: i32) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "edit" = $1 AND "area" = $2"#)
            .bind(edit)
            .bind(area)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_edit_and_area_list<'e, E: PgExecutor<'e>>(&self, executor: E, edit_list: Vec<i32>, area_list: Vec<i32>) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "edit" = ANY($1) AND "area" = ANY($2)"#)
            .bind(edit_list)
            .bind(area_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_edit_and_area_optional<'e, E: PgExecutor<'e>>(&self, executor: E, edit: i32, area: i32) -> Result<Option<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "edit" = $1 AND "area" = $2"#)
            .bind(edit)
            .bind(area)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE edit = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_area_id<'e, E: PgExecutor<'e>>(executor: E, area_id: i32) -> Result<Vec<EditArea>> {
        query_as::<_, EditArea>(r#"SELECT * FROM "musicbrainz"."edit_area" WHERE area = $1"#)
            .bind(area_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, edit_area: EditArea) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"INSERT INTO "edit_area" ("edit", "area") VALUES ($1, $2) RETURNING *;"#)
            .bind(edit_area.edit)
            .bind(edit_area.area)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, edit_area: EditArea) -> Result<EditArea> {
        query_as::<_, EditArea>(r#"UPDATE "edit_area" SET  WHERE "edit" = 1 AND "area" = 2 RETURNING *;"#)
            .bind(edit_area.edit)
            .bind(edit_area.area)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."edit_area" WHERE "edit" = 1 AND "area" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
