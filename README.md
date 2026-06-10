# Raphael Nembhard Personal Site

A Rust + Axum personal site with a Windows 95-inspired interface.

## Run

```bash
cargo run
```

Then open <http://127.0.0.1:3000>.

## Production-style Run

Local defaults are `HOST=127.0.0.1` and `PORT=3000`.

```bash
HOST=0.0.0.0 PORT=3000 cargo run --release
```

Health check:

```bash
curl http://127.0.0.1:3000/healthz
```

## Edit Content

Homepage rendering lives in `src/views.rs`, routes live in `src/app.rs` and `src/handlers.rs`, and static assets live in `assets/`.
