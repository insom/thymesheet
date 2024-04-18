use crate::models::Week;
use crate::schema::weeks::dsl::*;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use rocket::{get, State};
use rocket_dyn_templates::{context, Template};

use rocket::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    foo: String,
}

#[get("/")]
pub fn index(config: &State<Config>) -> Template {
    let mut connection = crate::establish_connection();
    let results = weeks
        .select(Week::as_select())
        .load(&mut connection)
        .unwrap();

    Template::render(
        "index",
        context! {weeks: &results, admin: false, foo: config.foo.clone()},
    )
}

#[get("/week/<week>")]
pub fn week(week: i32) -> Result<Template, NotFound<String>> {
    let mut connection = crate::establish_connection();
    let results: Vec<_> = weeks
        .select(Week::as_select())
        .filter(id.eq(week))
        .load(&mut connection)
        .unwrap();

    if results.len() == 0 {
        return Err(NotFound("Week Not Found".to_string()));
    }

    Ok(Template::render(
        "index",
        context! {weeks: &results, admin: false},
    ))
}
