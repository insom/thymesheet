use crate::schema::weeks::dsl::*;
use crate::{admin::AdminUser, models::Week, Thymesheet};
use diesel::prelude::*;
use rocket::get;
use rocket::response::status::NotFound;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(admin: Option<AdminUser>) -> Template {
    let mut connection = crate::establish_connection();
    let results = weeks
        .select(Week::as_select())
        .load(&mut connection)
        .unwrap();

    Template::render("index", context! {weeks: &results, admin: admin.is_some()})
}

#[get("/week/<week>")]
pub async fn week(week: i32, admin: Option<AdminUser>, db: Thymesheet) -> Result<Template, NotFound<String>> {
    db.run(|connection| {
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
        context! {weeks: &results, admin: admin.is_some(), title: format!("Week {}", week)},
    ))
    }).await
}
