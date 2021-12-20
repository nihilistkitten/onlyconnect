mod db;
mod error;

use error::Error;
use serde::Serialize;

use std::collections::HashMap;

#[macro_use]
extern crate diesel;

use db::{
    models::{NewUser, User},
    OCDBConn,
};

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::figment::{
    map,
    value::{Map, Value},
};
use rocket::response::status::Created;
use rocket::serde::msgpack::MsgPack;
use rocket::{get, post, routes, Build, Rocket};
use rocket_dyn_templates::Template;

#[get("/<name>")]
fn index(name: &str) -> Template {
    let context = HashMap::from([("name", name)]);
    Template::render("index", context)
}

#[derive(Serialize)]
struct NewUserResponse {
    user: User,
}

#[post("/user", format = "msgpack", data = "<user>")]
async fn new_user(
    conn: OCDBConn,
    user: MsgPack<NewUser>,
) -> Result<Created<MsgPack<NewUserResponse>>, Error> {
    let user: User = conn
        .run(move |c| {
            diesel::insert_into(db::schema::users::table)
                .values(user.into_inner())
                .get_result(c)
        })
        .await?;

    let response = NewUserResponse { user };
    Ok(Created::new("/user").body(MsgPack(response)))
}

/// Configure the Rocket instance using a local .env file.
fn rocket_config() -> rocket::figment::Figment {
    // load environment variables
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("no database url found: is the `.env` file in the project root?");
    let db_pool_size: u32 = std::env::var("DATABASE_POOL_SIZE").map_or(10, |s| {
        s.parse().expect("DATABASE_POOL_SIZE must be a valid u32")
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

    rocket::custom(config)
        .attach(Template::fairing())
        .attach(OCDBConn::fairing())
        .mount("/", routes![index])
        .mount("/api", routes![new_user])
}
