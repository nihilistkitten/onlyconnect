mod db;

use std::collections::HashMap;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::figment::{
    map,
    value::{Map, Value},
};
use rocket::{get, routes, Build, Rocket};
use rocket_dyn_templates::Template;

#[get("/<name>")]
fn index(name: &str) -> Template {
    let context = HashMap::from([("name", name)]);
    Template::render("index", context)
}

/// Configure the Rocket instance using a local .env file.
fn rocket_config() -> rocket::figment::Figment {
    // load environment variables
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("no database url found: is the `.env` file in the project root?");
    let db_pool_size: u32 = std::env::var("DATABASE_POOL_SIZE").map_or(10, |s| {
        s.parse()
            .expect("DATABASE_POOL_SIZE pool size must be a valid u32")
    });

    let db_conf: Map<_, Map<_, Value>> = map! ["ocdb" => map! {
        "url" => db_url.into(),
        "pool_size" => db_pool_size.into()
    }];

    rocket::Config::figment().merge(("databases", db_conf))
}

/// Build and launch the rocket instance.
#[must_use]
pub fn launch() -> Rocket<Build> {
    let config = rocket_config();

    dbg!(&config);

    rocket::custom(config)
        .attach(Template::fairing())
        .attach(db::OCDBConn::fairing())
        .mount("/", routes![index])
}
