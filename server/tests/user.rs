//! Tests for the user route.
use chrono::{Duration, DurationRound};
use db::models::{NewUser, User};

use rocket::http::Status;
use rocket::local::blocking::{Client, LocalResponse};

use server::launch;

struct TestDb;

impl TestDb {
    fn new() -> Self {
        dotenv::dotenv().ok();

        // load the test database url
        std::env::set_var(
            "DATABASE_URL",
            std::env::var("TEST_DATABASE_URL").expect("$TEST_DATABASE_URL must be set for testing"),
        );

        Self
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let conn: diesel::PgConnection =
            diesel::Connection::establish(&std::env::var("TEST_DATABASE_URL").unwrap())
                .expect("the test database must exist");

        loop {
            // revert the migration until it's completely reverted, so we work with a fresh test db
            // next time
            if diesel_migrations::revert_latest_migration_in_directory(
                &conn,
                std::path::Path::new("../db/migrations"),
            )
            .is_err()
            {
                break;
            }
        }
    }
}

fn get_test_client() -> Client {
    let rocket = launch();
    Client::tracked(rocket).unwrap()
}

/// Assert that a user is equivalent to a new user.
fn assert_user_eq_without_id(u: &User, nu: &NewUser) {
    assert_eq!(u.email, nu.email);
    assert_eq!(u.username, nu.username);
    assert_eq!(
        u.updated_at,
        nu.updated_at
            .duration_trunc(Duration::microseconds(1))
            .unwrap()
    );
}

fn insert_user<'a>(nu: &NewUser, client: &'a Client) -> LocalResponse<'a> {
    let req = client.post("/api/user").msgpack(nu);
    req.dispatch()
}

#[test]
#[ignore]
fn new_user_works() {
    let _test = TestDb::new();

    let nu = NewUser {
        email: "test@example.org".into(),
        username: "test".into(),
        updated_at: chrono::Utc::now(),
    };

    let client = get_test_client();
    let response = insert_user(&nu, &client);

    assert_eq!(response.status(), Status::Created);
    let response_user = response
        .into_msgpack::<User>()
        .expect("request returns msgpack data");

    assert_user_eq_without_id(&response_user, &nu);
}

#[test]
#[ignore]
fn duplicate_users_fail() {
    let _test = TestDb::new();

    let nu = NewUser {
        email: "test@example.org".into(),
        username: "test".into(),
        updated_at: chrono::Utc::now(),
    };

    let client = get_test_client();
    insert_user(&nu, &client);
    let response = insert_user(&nu, &client);

    assert_eq!(response.status(), Status::InternalServerError);
}

#[test]
#[ignore]
fn multiple_users_work() {
    let _test = TestDb::new();

    let nu = NewUser {
        email: "test@example.org".into(),
        username: "test".into(),
        updated_at: chrono::Utc::now(),
    };

    let client = get_test_client();
    insert_user(&nu, &client);

    let nu2 = NewUser {
        email: "test2@example.org".into(),
        username: "test2".into(),
        updated_at: chrono::Utc::now(),
    };

    let response = insert_user(&nu2, &client);

    assert_eq!(response.status(), Status::Created);
    let response_user = response
        .into_msgpack::<User>()
        .expect("request returns msgpack data");

    assert_user_eq_without_id(&response_user, &nu2);
}
