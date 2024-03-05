#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::ArtistType;

pub struct ArtistTypeSet;

impl ArtistTypeSet {
// SELECT statements
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<ArtistType>> {
        query_as::<_, ArtistType>(r#"SELECT * FROM "musicbrainz"."artist_type""#)
            .fetch_all(executor)
            .await
    }

// SELECT By Primary Key statements
    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<ArtistType> {
        query_as::<_, ArtistType>(r#"SELECT * FROM "musicbrainz"."artist_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

// SELECT many by Primary Key statements
    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<ArtistType>> {
        query_as::<_, ArtistType>(r#"SELECT * FROM "musicbrainz"."artist_type" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

// SELECT by Primary Key statements
    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<ArtistType>> {
        query_as::<_, ArtistType>(r#"SELECT * FROM "musicbrainz"."artist_type" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

// SELECT statements by Foreign Key statements
    pub async fn all_by_artist_type_id_where_parent_is<'e, E: PgExecutor<'e>>(executor: E, artist_type_id: i32) -> Result<Vec<ArtistType>> {
        query_as::<_, ArtistType>(r#"SELECT * FROM "musicbrainz"."artist_type" WHERE parent = $1"#)
            .bind(artist_type_id)
            .fetch_all(executor)
            .await
    }

// INSERT statements
    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, artist_type: ArtistType) -> Result<ArtistType> {
        query_as::<_, ArtistType>(r#"INSERT INTO "artist_type" ("id", "name", "parent", "child_order", "description", "gid") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(artist_type.gid)
            .bind(artist_type.name)
            .bind(artist_type.parent)
            .bind(artist_type.child_order)
            .bind(artist_type.description)
            .bind(artist_type.id)
            .fetch_one(executor)
            .await
    }

// UPDATE statements
    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, artist_type: ArtistType) -> Result<ArtistType> {
        query_as::<_, ArtistType>(r#"UPDATE "artist_type" SET "parent" = $3, "name" = $2, "description" = $5, "child_order" = $4, "gid" = $6 WHERE "id" = 1 RETURNING *;"#)
            .bind(artist_type.child_order)
            .bind(artist_type.name)
            .bind(artist_type.parent)
            .bind(artist_type.description)
            .bind(artist_type.gid)
            .bind(artist_type.id)
            .fetch_one(executor)
            .await
    }

// DELETE statements
    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "musicbrainz"."artist_type" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
