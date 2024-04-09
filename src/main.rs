#![allow(unused_imports)]

#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use thymesheet::models::*;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let v = Vec::<bool>::new(); // an empty vec needs to have type information so it can be serailized #smh
    Template::render("index", context! {weeks: &v})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
