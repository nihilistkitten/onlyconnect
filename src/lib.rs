use std::collections::HashMap;

use rocket::{get, routes, Build, Rocket};
use rocket_dyn_templates::Template;

#[get("/<name>")]
fn index(name: &str) -> Template {
    let context = HashMap::from([("name", name)]);
    Template::render("index", context)
}

#[must_use]
pub fn launch() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
