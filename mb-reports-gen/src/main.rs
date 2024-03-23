pub mod types;
use mb_rs::schema::Artist;
use sqlx::{Column, Error, PgPool, Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use crate::types::ArtistsThatMayBeGroups;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = create_pg_connection().await.ok().unwrap();
    artists_that_maybe_groups(pool).await;
    Ok(())
}


async fn create_pg_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://musicbrainz:musicbrainz@localhost:5432/musicbrainz_db")
        .await;
    return pool

}

async fn artists_that_maybe_groups(pg_pool: PgPool) {
    artist_join_l_artist_artist(pg_pool).await;
}

async fn artist_join_l_artist_artist(pg_pool: PgPool) {
    let artist = sqlx::query_as::<_, ArtistsThatMayBeGroups>(
        "
          WITH possible_groups_entity0 AS (
         SELECT artist.id, artist.name
         FROM artist
         JOIN l_artist_artist laa ON laa.entity0 = artist.id
         JOIN link ON link.id = laa.link
         WHERE (
            artist.type NOT IN (2, 5, 6) -- group, orchestra, choir
            OR
            artist.type IS NULL
         )
         AND link.link_type IN (722)), -- subgroup
       possible_groups_entity1 AS (
         SELECT artist.id, artist.name
         FROM artist
         JOIN l_artist_artist laa ON laa.entity1 = artist.id
         JOIN link ON link.id = laa.link
         JOIN link_type ON link_type.id = link.link_type
         WHERE (
            artist.type NOT IN (2, 5, 6) -- group, orchestra, choir
            OR
            artist.type IS NULL
         )
         AND link.link_type IN (
            103, -- member of band
            102, -- collaboration
            305, -- conductor position
            965, -- artistic director
            855, -- composer-in-residence
            722  -- subgroup
         )
       )
  SELECT  possible_groups.id AS artist_id,
         row_number() OVER (ORDER BY possible_groups.name COLLATE musicbrainz, possible_groups.id)
  FROM
    (SELECT * FROM possible_groups_entity0
        UNION
     SELECT * FROM possible_groups_entity1) AS possible_groups
        "
    ).fetch_all(&pg_pool).await.unwrap();
    println!("Will it work?");
    println!("{:?}", artist);
}