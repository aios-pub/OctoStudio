# OctoStudio v0.6 native — agent instructions

This is the Rust + Makepad rewrite of OctoStudio. Read `README.md` first.

## Build / run

```bash
cd native
cargo build --release
./target/release/octostudio --port 8141
```

If a build fails, read the error carefully — the `script_mod!` macro
uses lexical scope (widgets defined in one crate's `script_mod!` block
cannot be referenced from another), so cross-crate widget references
must be inlined into a single `script_mod!` block (see `app.rs`).

## Code style

- `script_mod!` widgets are inlined in `app.rs` (1 file, ~700 lines).
  Helpers go in `crates/octostudio-*/` (Rust modules).
- All state in a global `static APP_STATE: OnceLock<Mutex<AppState>>`,
  because `#[derive(Script)]` on `App` requires every field to be
  `#[live]` (script-decorated).
- Click handler pattern: `if self.ui.button(cx, ids!(X)).clicked(actions) { ... }`.
  Plain `View` widgets don't have a `clicked()` action — use
  `ButtonFlat` for clickable rows.
- HTTP: synchronous `ureq` calls in `crates/octostudio-ai/src/client.rs`.
  Real async streaming will need `cx.http_request` (out of scope for v0.6 alpha).

## API conventions

- Agnes 3.0 Flash chat: `POST /v1/chat/completions` with `Authorization: Bearer <key>`
  and optional `response_format: { type: "json_schema", ... }` for structured output.
- Agnes 2.5 image: `POST /v1/images/generations` with `extra_body: { response_format: "url" }`.
- Agnes 2.5 video: `POST /v1/videos` returns `{ video_id, status }`; poll
  `GET /agnesapi?video_id=...&model_name=agnes-video-2.5` until `status == "completed"`.

## Testing

- 25 unit tests across 6 crates (`cargo test`).
- For end-to-end visual verification, use Makepad Studio:
  `curl 127.0.0.1:8141/snap` (widget tree) and
  `curl '127.0.0.1:8141/g?raw=1' -o shot.png` (PNG capture).
