use dotenv::dotenv;
use rocket::figment::value::{Map, Value};
use rocket::figment::{map, Figment};

/// Generate the Rocket configuration using a local .env file.
pub fn config() -> Figment {
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
