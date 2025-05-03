use diesel::{
    prelude::{Insertable, Queryable},
    QueryResult, Selectable,
};
use rspotify::{model::PrivateUser, prelude::Id};

use crate::schema::person;

use super::PoolConnection;

#[derive(Insertable)]
#[diesel(table_name = person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct InsertablePerson {
    pub spotify_id: String,
    pub name: Option<String>,
}

pub async fn create_new_user(
    mut connection: PoolConnection,
    spotify_user: PrivateUser,
) -> QueryResult<bool> {
    use diesel::prelude::*;

    let new_person = InsertablePerson {
        spotify_id: spotify_user.id.id().to_string(),
        name: spotify_user.display_name,
    };

    let response = diesel::insert_into(person::table)
        .values(new_person)
        .on_conflict(person::spotify_id)
        .do_nothing()
        .execute(&mut connection)?;
    if response == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[diesel(table_name = person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SelectablePerson {
    pub id: i32,
    pub spotify_id: String,
    pub name: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_user_by_spotify_id(
    mut connection: PoolConnection,
    spotify_id: String,
) -> QueryResult<SelectablePerson> {
    use diesel::prelude::*;

    person::table
        .filter(person::spotify_id.eq(spotify_id))
        .select(SelectablePerson::as_select())
        .first(&mut connection)
}
