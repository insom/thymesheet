use crate::Thymesheet;
use chrono::{prelude::*, TimeDelta};
use rocket::response::status::NotFound;
use rocket::State;
use rocket::{get, http::ContentType};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use rss::{ChannelBuilder, ItemBuilder};

#[get("/")]
pub async fn index(db: Connection<Thymesheet>) -> Template {
    let results = crate::get_weeks(db).await;
    Template::render("index", context! {weeks: &results, admin: false})
}

#[get("/week/<week>")]
pub async fn week(week: i32, db: Connection<Thymesheet>) -> Result<Template, NotFound<String>> {
    crate::render_week(week, String::from("index"), db).await
}

#[get("/rss")]
pub async fn rss_feed(
    db: Connection<Thymesheet>,
    config: &State<crate::Config>,
) -> (rocket::http::ContentType, String) {
    let results = crate::get_weeks(db).await;
    let mut chan = ChannelBuilder::default();
    chan.title("Thymesheet")
        .link(config.base_url.as_str())
        .description("Days and Weeks");
    for result in results {
        let mut date = chrono::Utc
            .with_ymd_and_hms(result.id / 100, 1, 1, 0, 0, 1)
            .unwrap();
        date = date + TimeDelta::try_weeks((result.id % 100) as i64).unwrap();
        if date > chrono::Utc::now() {
            // Don't add the current week to RSS -- we're not done filling it in, yet.
            continue;
        }
        let mut item = ItemBuilder::default();
        item.link(format!("{}/week/{}", config.base_url, result.id));
        item.description(markdown::to_html(result.body.as_str()));
        item.pub_date(Some(String::from(date.to_rfc2822())));
        chan.item(item.build());
    }
    (ContentType::XML, chan.build().to_string())
}
