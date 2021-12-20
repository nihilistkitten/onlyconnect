# Development

## CLI Testing

```
curl -X POST http://localhost:8000/api/user -H "Content-Type: application/msgpack" -d (echo '{"username": "test", "email": "example@gmail.com", "updated_at": "'(date -Iseconds)'"}' | json2msgpack) | msgpack2json
```

## Diesel

Create a `.env` file in the project root with e.x. `DATABASE_URL=postgres://[username]:[password]@localhost/ocdb`, or the appropriate url to your local postgres database. One that's done, `diesel setup` should set the database up properly.
