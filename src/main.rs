#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![thymesheet::public::index])
        .mount("/", routes![thymesheet::admin::index])
        .mount("/", routes![thymesheet::admin::login])
        .attach(Template::fairing())
}
