use rocket::get;
#[allow(unused)]
use diesel::prelude::*;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    let v = Vec::<bool>::new(); // an empty vec needs to have type information so it can be serailized #smh
    Template::render("index", context! {weeks: &v, admin: false})
}
