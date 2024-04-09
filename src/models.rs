use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::days)]
pub struct Day {
    pub id: i32,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::weeks)]
pub struct Week {
    pub id: i32,
    pub body: String,
    pub published: bool,
}
