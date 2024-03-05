#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Annotation;

pub struct AnnotationSet;

impl AnnotationSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Annotation>> {
        query_as::<_, Annotation>(r#"SELECT * FROM "musicbrainz"."annotation""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Annotation> {
        query_as::<_, Annotation>(r#"SELECT * FROM "musicbrainz"."annotation" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<Annotation>> {
        query_as::<_, Annotation>(r#"SELECT * FROM "musicbrainz"."annotation" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<Annotation>> {
        query_as::<_, Annotation>(r#"SELECT * FROM "musicbrainz"."annotation" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_editor_id_where_editor_is<'e, E: PgExecutor<'e>>(executor: E, editor_id: i32) -> Result<Vec<Annotation>> {
        query_as::<_, Annotation>(r#"SELECT * FROM "musicbrainz"."annotation" WHERE editor = $1"#)
            .bind(editor_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, annotation: Annotation) -> Result<Annotation> {
        query_as::<_, Annotation>(r#"INSERT INTO "annotation" ("id", "editor", "text", "changelog", "created") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(annotation.created)
            .bind(annotation.id)
            .bind(annotation.text)
            .bind(annotation.changelog)
            .bind(annotation.editor)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, annotation: Annotation) -> Result<Annotation> {
        query_as::<_, Annotation>(r#"UPDATE "annotation" SET "editor" = $2, "created" = $5, "text" = $3, "changelog" = $4 WHERE "id" = 1 RETURNING *;"#)
            .bind(annotation.text)
            .bind(annotation.id)
            .bind(annotation.created)
            .bind(annotation.editor)
            .bind(annotation.changelog)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."annotation" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
