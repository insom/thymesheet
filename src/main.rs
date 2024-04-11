#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use thymesheet::{admin, public};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![public::index])
        .mount("/admin", routes![admin::index, admin::login, admin::logout, admin::login_get])
        .register("/admin", catchers![admin::redir_to_login])
        .attach(Template::fairing())
}
