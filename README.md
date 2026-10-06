# my-docker-sandbox-test

Dummy web app built with [Rocket](https://rocket.rs) to explore Docker sandboxes and Claude Code.

## Run

```bash
cargo run          # listens on http://127.0.0.1:8000
cargo test
```

Set `ROCKET_ADDRESS=0.0.0.0` to listen on all interfaces (e.g. inside a container).

## Endpoints

| Method | Path      | Body                | Response                        |
|--------|-----------|---------------------|---------------------------------|
| GET    | `/health` | –                   | `{"status":"ok"}`               |
| POST   | `/greet`  | `{"name":"Alice"}`  | `{"message":"Hello, Alice!"}`   |

```bash
curl http://127.0.0.1:8000/health
curl -X POST http://127.0.0.1:8000/greet -H 'Content-Type: application/json' -d '{"name":"Alice"}'
```
