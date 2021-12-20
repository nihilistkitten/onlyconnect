use rocket::launch;
use server::launch;

#[launch]
fn rocket() -> _ {
    launch()
}
