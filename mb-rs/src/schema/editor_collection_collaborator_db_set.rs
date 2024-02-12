#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::EditorCollectionCollaborator;

pub struct EditorCollectionCollaboratorSet;

impl EditorCollectionCollaboratorSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_editor<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, editor: i32) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "collection" = $1 AND "editor" = $2"#)
            .bind(collection)
            .bind(editor)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_collection_and_editor_list<'e, E: PgExecutor<'e>>(&self, executor: E, collection_list: Vec<i32>, editor_list: Vec<i32>) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "collection" = ANY($1) AND "editor" = ANY($2)"#)
            .bind(collection_list)
            .bind(editor_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_collection_and_editor_optional<'e, E: PgExecutor<'e>>(&self, executor: E, collection: i32, editor: i32) -> Result<Option<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "collection" = $1 AND "editor" = $2"#)
            .bind(collection)
            .bind(editor)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_release<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_release_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_release_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_medium<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_medium_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_medium_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_position<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_position_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_position_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_editor_collection_id<'e, E: PgExecutor<'e>>(executor: E, editor_collection_id: i32) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE collection = $1"#)
            .bind(editor_collection_id)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_editor_id<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<EditorCollectionCollaborator>> {
        query_as::<_, EditorCollectionCollaborator>(r#"SELECT * FROM "musicbrainz"."editor_collection_collaborator" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_collaborator: EditorCollectionCollaborator) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"INSERT INTO "editor_collection_collaborator" ("collection", "editor") VALUES ($1, $2) RETURNING *;"#)
            .bind(editor_collection_collaborator.collection)
            .bind(editor_collection_collaborator.editor)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, editor_collection_collaborator: EditorCollectionCollaborator) -> Result<EditorCollectionCollaborator> {
        query_as::<_, EditorCollectionCollaborator>(r#"UPDATE "editor_collection_collaborator" SET  WHERE "collection" = 1 AND "editor" = 2 RETURNING *;"#)
            .bind(editor_collection_collaborator.collection)
            .bind(editor_collection_collaborator.editor)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."editor_collection_collaborator" WHERE "collection" = 1 AND "editor" = 2"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
