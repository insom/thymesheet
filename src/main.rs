#[macro_use]
extern crate rocket;

use markdown;
use rocket_dyn_templates::handlebars::*;
use rocket_dyn_templates::Template;
use thymesheet::{admin, public};

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
        .mount("/", routes![public::index, public::week])
        .mount(
            "/admin",
            routes![admin::index, admin::login, admin::logout, admin::login_get],
        )
        .register("/admin", catchers![admin::redir_to_login])
        .attach(Template::custom(
            |engines: &mut rocket_dyn_templates::Engines| {
                engines
                    .handlebars
                    .register_helper("markdown", Box::new(markdownize))
            },
        ))
}
