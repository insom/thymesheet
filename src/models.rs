use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::days)]
pub struct Day {
    pub id: i32,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, rocket::serde::Serialize)]
#[diesel(table_name = crate::schema::weeks)]
pub struct Week {
    pub id: i32,
    pub body: String,
    pub published: bool,
}

#[derive(AsChangeset, Identifiable, Clone, Copy, Insertable)]
#[derive(rocket::form::FromForm)]
#[diesel(table_name = crate::schema::weeks)]
pub struct WeekForm<'a> {
    pub id: i32,
    pub body: &'a str,
}
