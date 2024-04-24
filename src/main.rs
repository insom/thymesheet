#[macro_use]
extern crate rocket;

use markdown;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket_dyn_templates::handlebars::*;
use rocket_dyn_templates::Template;
use thymesheet::{public, admin};
use rocket_db_pools::Database;

fn markdownize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap().value().render();

    out.write(&markdown::to_html(param.as_ref()))?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(thymesheet::Thymesheet::init())
        .mount("/public", FileServer::from("static/"))
        .mount("/", routes![public::index, public::week])
        .mount(
            "/admin",
            routes![
                admin::index,
                admin::login,
                admin::logout,
                admin::login_get,
                admin::week,
                admin::week_get,
            ],
        )
        .register("/admin", catchers![admin::redir_to_login])
        .attach(AdHoc::config::<thymesheet::Config>())
        .attach(Template::custom(
            |engines: &mut rocket_dyn_templates::Engines| {
                engines
                    .handlebars
                    .register_helper("markdown", Box::new(markdownize))
            },
        ))
}
