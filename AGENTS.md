# OctoStudio v0.6 — agent instructions

> OctoStudio 已是**原生 Makepad Rust 应用**。Splash 脚本、card-host、OctoSense App Hub、
> Rinx mini-app 全部脱钩。任何把它们拽回来的 patch 都要拒绝。

## 仓库形状

| 路径 | 角色 |
|---|---|
| `Cargo.toml` | workspace 根(8 crate + `makepad-widgets` path dep) |
| `crates/octostudio-{core,theme,ai,export,storage,render,demo,app}` | 业务 crate 各管一摊 |
| `native/` | v0.6 文档(README + AGENTS.md + screenshots) |
| `quick_start.md` | **从 0 跑通 + 调试** — 所有 agent 第一站 |
| `README.md` | 用户向发布摘要 |
| `LICENSE` | Apache-2.0 |
| `build/keys/` | 本地 anchor 密钥(gitignored,勿入 git) |
| `target/` | Cargo build output(已 gitignore) |
| ~~`bundle/`~~, ~~`build/`~~, ~~`BRIEF.md`~~ 等 | splash 时代,已移除 |

`makepad-widgets` 解析到 `path = "../makepad/widgets"`(本仓在 `octostudio/`、`makepad/` 是同级目录)。

## 一切从这里开始

1. 先读 [`quick_start.md`](quick_start.md) — 编译、运行、Studio 模式、调试、测试
2. 再读 [`native/README.md`](native/README.md) — 8 crate 职责切分 + 屏幕 + AI 表
3. 开发时读 [`native/AGENTS.md`](native/AGENTS.md) — `script_mod!` 宏 / state pattern / id 约定

## 写代码的硬约束

- **不引** `tokio` / `async-std` / `reqwest` — HTTP 走同步 `ureq`(`crates/octostudio-ai/src/client.rs`)
- **不写** `.splash` 脚本或 Splash VM 引用
- **不改** splash 时代字段名(`plan_sections` / `plan_scenes`)— 迁移在 `octostudio-storage/src/migration.rs`
- **不申请** splash `manifest.json` 那一坨 capability;auth 只用 `Authorization: Bearer <key>`
- **API 调用全部 OpenAI 兼容**:`POST /v1/chat/completions` + `POST /v1/images/generations` + `POST /v1/videos` + `GET /agnesapi?…`
- **70s AI 看门狗**是 `handle_watchdog_timer_fire` 的事;每次新加 AI 调用都要走它
- **新加 widget** 在 `crates/octostudio-app/src/app.rs` 顶层 `script_mod!` 块,**不要**拆到其它 crate

## 提交流程

每个里程碑(屏幕 / AI / 导出 / 设置)一个 commit。commit message 模板:

```
v0.6 C{数字} {类别}: {具体改}

- {变更 1}
- {变更 2}

Tests: cargo test --workspace ||  31 passed
Build: cargo build --release -p octostudio-app ||  Finished in Xs
```

PR / push 之前必跑一遍 `cargo check --workspace`、`cargo test --workspace`、再抓一张目标屏
PNG(`./target/release/octostudio --remote=8141` 后 `curl '127.0.0.1:8141/g?raw=1' -o xxx.png`)
放到 `native/screenshots/`。

## 显式不做

- wasm / 移动端(只桌面 macOS arm64 + linux x86_64)
- Splash VM 兼容层 / card-host 加载 splash
- protobuf / 自定义协议(全 OpenAI 兼容)
- 多用户 / 账号 / 同步
- 暗色模式全套(只预留 token)
- Splash 旧 capability 接入(`/octos.turn.start`、`/glance.publish` 等)
