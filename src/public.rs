use crate::models::Week;
use crate::schema::weeks::dsl::*;
use diesel::prelude::*;
use rocket::get;
use rocket::response::status::NotFound;
use rocket_dyn_templates::{context, Template};
use markdown;

#[get("/")]
pub fn index() -> Template {
    let mut connection = crate::establish_connection();
    let results = weeks
        .select(Week::as_select())
        .load(&mut connection)
        .unwrap();

    Template::render("index", context! {weeks: &results, admin: false})
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
    let better_results: Vec<_> = results.iter().map(|w| markdown::to_html(&w.body)).collect();

    Ok(Template::render(
        "index",
        context! {weeks: &better_results, admin: false},
    ))
}
