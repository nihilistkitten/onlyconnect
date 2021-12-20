mod config;
mod db_bindings;
mod error;

use std::collections::HashMap;

#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::serde::msgpack::MsgPack;
use rocket::{get, post, routes, Build, Rocket};
use rocket_dyn_templates::Template;

use config::config;
use db::models::{NewUser, User};
use db_bindings::{migrate, OcdbConn};
use error::Error;

#[get("/<name>")]
fn index(name: &str) -> Template {
    let context = HashMap::from([("name", name)]);
    Template::render("index", context)
}

#[post("/user", format = "msgpack", data = "<user>")]
async fn new_user(conn: OcdbConn, user: MsgPack<NewUser>) -> Result<Created<MsgPack<User>>, Error> {
    let user: User = conn
        .run(move |c| {
            diesel::insert_into(db::schema::users::table)
                .values(user.into_inner())
                .get_result(c)
        })
        .await?;

    Ok(Created::new("/user").body(MsgPack(user)))
}

/// Build and launch the rocket instance.
#[must_use]
pub fn launch() -> Rocket<Build> {
    let config = config();

    rocket::custom(config)
        .attach(Template::fairing())
        .attach(OcdbConn::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", migrate))
        .mount("/", routes![index])
        .mount("/api", routes![new_user])
}
