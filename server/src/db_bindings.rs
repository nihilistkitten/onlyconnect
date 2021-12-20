use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel::PgConnection};

#[database("ocdb")]
pub struct OcdbConn(PgConnection);

diesel_migrations::embed_migrations!("../db/migrations");

/// Run diesel migrations.
pub async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = OcdbConn::get_one(&rocket)
        .await
        .expect("database connection");

    db.run(|conn| match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            rocket::error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}
