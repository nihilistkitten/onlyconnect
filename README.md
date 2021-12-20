# Database Stuff

## Configuration

Create a `.env` file in the project root with e.x.

```sh
DATABASE_URL=postgres://[username]:[password]@localhost/ocdb
TEST_DATABASE_URL=postgres://[username]:[password]@localhost/ocdb_test
```

or the appropriate url to your local postgres database. One that's done, `diesel setup` should set the database up properly.

## Testing

Some integration tests depend on a test database connection. These are ignored by default, since they must be run in sequence, and should be run like:

```sh
cargo test -- --ignored --test-threads=1
```
