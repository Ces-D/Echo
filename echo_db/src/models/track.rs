use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SimpleTrack {
    pub id: i32,
    pub name: String,
    pub spotify_id: Option<String>,
    pub isrc: Option<String>,
    pub user_rating: Option<i16>,
}
