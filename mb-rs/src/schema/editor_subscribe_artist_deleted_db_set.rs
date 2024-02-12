#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorSubscribeArtistDeleted;

pub struct EditorSubscribeArtistDeletedSet;

impl EditorSubscribeArtistDeletedSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_editor_and_gid<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, gid: uuid::Uuid) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "editor" = $1 AND "gid" = $2"#)
            .bind(editor)
            .bind(gid)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_editor_and_gid_list<'e, E: PgExecutor<'e>>(&self, executor: E, editor_list: Vec<i32>, gid_list: Vec<uuid::Uuid>) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "editor" = ANY($1) AND "gid" = ANY($2)"#)
            .bind(editor_list)
            .bind(gid_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_editor_and_gid_optional<'e, E: PgExecutor<'e>>(&self, executor: E, editor: i32, gid: uuid::Uuid) -> Result<Option<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "editor" = $1 AND "gid" = $2"#)
            .bind(editor)
            .bind(gid)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_deleted_entity_gid<'e, E: PgExecutor<'e>>(executor: E, deleted_entity_gid: uuid::Uuid) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE gid = $1"#)
            .bind(deleted_entity_gid)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_edit_id<'e, E: PgExecutor<'e>>(executor: E, edit_id: i32) -> Result<Vec<EditorSubscribeArtistDeleted>> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"SELECT * FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE deleted_by = $1"#)
            .bind(edit_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_artist_deleted: EditorSubscribeArtistDeleted) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"INSERT INTO "editor_subscribe_artist_deleted" ("editor", "gid", "deleted_by") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(editor_subscribe_artist_deleted.editor)
            .bind(editor_subscribe_artist_deleted.gid)
            .bind(editor_subscribe_artist_deleted.deleted_by)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_subscribe_artist_deleted: EditorSubscribeArtistDeleted) -> Result<EditorSubscribeArtistDeleted> {
        query_as::<_, EditorSubscribeArtistDeleted>(r#"UPDATE "editor_subscribe_artist_deleted" SET "deleted_by" = $3 WHERE "editor" = 1 AND "gid" = 2 RETURNING *;"#)
            .bind(editor_subscribe_artist_deleted.editor)
            .bind(editor_subscribe_artist_deleted.gid)
            .bind(editor_subscribe_artist_deleted.deleted_by)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_subscribe_artist_deleted" WHERE "editor" = 1 AND "gid" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
