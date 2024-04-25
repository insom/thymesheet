use crate::Thymesheet;
use rocket::get;
use rocket::response::status::NotFound;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index(db: Connection<Thymesheet>) -> Template {
    let results = crate::get_weeks(db).await;
    Template::render("index", context! {weeks: &results, admin: false})
}

#[get("/week/<week>")]
pub async fn week(week: i32, db: Connection<Thymesheet>) -> Result<Template, NotFound<String>> {
    crate::render_week(week, String::from("index"), db).await
}
