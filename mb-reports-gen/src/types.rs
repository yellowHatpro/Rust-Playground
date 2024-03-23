#[derive(sqlx::FromRow, Debug)]
pub struct ArtistsThatMayBeGroups {
    pub artist_id: Option<i32>,
    pub row_number: Option<i64>,
}