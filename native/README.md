# OctoStudio v0.6 — Native Makepad

A complete native rewrite of the OctoStudio content studio, dropping the
Splash VM / card-host / octosense / rinx stack in favour of Rust +
[Makepad](https://github.com/makepad/makepad) widgets, talking directly
to three real AI providers on [Agnes](https://www.agnes-ai.com):

| Model             | Use                                    | Format                  |
|-------------------|----------------------------------------|-------------------------|
| `agnes-3.0-flash`     | text — 7 panel helpers + 10 scenarios   | OpenAI `chat.completions` |
| `agnes-image-2.5-flash` | image — plan-view AI 配图            | OpenAI `images/generations` |
| `agnes-video-2.5`      | video — plan-view 合成视频(异步轮询)   | OpenAI Videos + `/agnesapi` |

## Workspace layout

8 crates at the repo root with clear responsibility division, sharing
types via `octostudio-core` / `octostudio-theme`:

```
.                                # repo root = workspace root (Cargo.toml here)
└── crates/
    ├── octostudio-app      # binary — main entry, App glue, 5-screen shell, TabBar
    ├── octostudio-core     # types — Work, PlanItem, Composition, Screen, PlanKind,
    │                        # ExportFormat, SortMode, SCENARIOS/THEMES/PRESETS/STYLES
    ├── octostudio-theme    # tokens — 16 colors + 7-step type scale (light + dark reservations)
    ├── octostudio-storage  # JSON IO — works.json / config.json / usage.json
    │                        # + splash legacy migration
    ├── octostudio-ai       # Agnes clients — text (Agnes 3.0), image (2.5), video (2.5)
    │                        # + 7 panel helpers + 10 scenario prompts/schemas
    ├── octostudio-export   # 8 format renderers — markdown / wechat / notion / srt /
    │                        # pack / marp / markmap
    ├── octostudio-render   # AppState — 35+ field mirror of splash `let` bindings
    └── octostudio-demo     # 2 first-launch demo works (测试作品一/二)
```

## Build

```bash
cargo build --release      # from the repo root (this workspace's root Cargo.toml)
./target/release/octostudio --remote=8141
```

The `makepad-widgets` dependency resolves via path to
`/Volumes/PSSD/CodeProjects/makepad` (see root `Cargo.toml`).

## Run

```bash
./target/release/octostudio                                  # default 1280x800 (hard-coded in app.rs startup)
./target/release/octostudio --remote=8141                    # + HTTP debug bridge on 127.0.0.1:8141
```

Window size is `window.inner_size: vec2(1280, 800)` in
`crates/octostudio-app/src/app.rs` — no CLI flag; edit and rebuild to change.

Full run/debug/Studio guide: [`../quick_start.md`](../quick_start.md).

### Remote debug bridge (makepad built-in)

Every binary can expose a localhost HTTP control surface with `--remote`
(`platform/src/remote.rs` in makepad). All routes are GET:

```bash
curl 127.0.0.1:8141/snap?q=          # widget rects, ready to click
curl '127.0.0.1:8141/g?raw=1' -o shot.png   # capture PNG
curl '127.0.0.1:8141/click?x=400&y=600&wait=1'  # tap (real event path)
curl '127.0.0.1:8141/t?text=hello'   # type into focused field
curl 127.0.0.1:8141/gq               # grab + graceful quit
```

## First launch

1. Open the app. Two demo works (`测试作品一` / `测试作品二`) are
   auto-seeded into `~/Library/Application Support/octostudio/works.json`.
2. Click ⚙ **设置** at the bottom-right.
3. Paste your Agnes API key in the input, click **保存**, then
   **测试连接** to verify with a 1-token ping. Any AI call from the
   Home / Plan / Studio / Export screens will now reach the real API.
4. Without a key, every AI button on Home still works — it falls back
   to demo data with a status-line hint.

## Tabs

| Tab | Feature | AI |
|-----|---------|----|
| ◐ 广场 (Home) | works library, search, sort, scenario grid, "🎨 测试 AI 配图" + "🎬 测试 AI 合成视频" + "📄 测试导出" buttons | image / video / export |
| ✎ 录入 (Compose) | scenario / theme / preset chips + 3 input cards | (text in C5 stub) |
| ◇ 工坊 (Studio) | 8 style chips + 7 AI panel chips + content editor | text (C5 stub) |
| ▤ 计划 (Plan) | plan items + AI 配图 + AI 合成视频 | (image / video in C6-C7 stubs) |
| ⚙ 设置 (Settings) | API key + 测试连接 + 主题 picker + 配额 stats | n/a |

`导出` is reached via the Plan/Studio top-right button (not a tab).

## Tests

```bash
cargo test                                  # 31 passing across all crates
cargo test -p octostudio-ai                 # text/image/video clients
cargo test -p octostudio-storage            # works.json / config.json / usage.json
cargo test -p octostudio-export             # 8 format renderers (srt time code roundtrip)
```

## Limitations (v0.6 alpha)

- No dark mode rendering (theme tokens reserved, switching is v0.6.1)
- AI calls are **synchronous on the UI thread** (1-30s blocking). A
  future C7.1 will move chat to `cx.http_request` streaming + a
  `Cx::start_timeout(2.0, repeat)` poll loop for video.
- Splash-era files (`bundle/`, old docs, promo media) were removed from
  the repo in v0.6 — see the history note in the root `README.md`.

## Migration from v0.5-alpha-ui-polish

The splash bundle (3052 lines of `.splash` script) is functionally
preserved 1:1. On first launch, v0.6 looks for
`~/.octosense/octostudio/works.json` and `<bundle>/.local-state/works.json`
and migrates any legacy entries (lifts `plan_sections` / `plan_scenes`
into canonical `plan_items`).
