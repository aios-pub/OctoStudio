# OctoStudio v0.6

> **言出法随,意到文成 ——「一句话」即「可发布产物」**

v0.6 是 **Rust + Makepad 原生桌面应用**。完整重写了 v0.5 之前的 Splash 脚本层,
直接对接 3 个真 [Agnes](https://www.agnes-ai.com) AI 模型(全部 OpenAI 兼容协议)。
**Splash VM / card-host / OctoSense / Rinx 全部脱钩** —— `./target/release/octostudio`
直接跑,不再有中间层。

## 5 屏

| # | Tab | 内容 | 真 AI |
|---|---|---|---|
| 1 | ◐ 广场(Home)| 作品库 · 搜索 · 排序 · 多选 · 9 场景入口 · 「🎨 测试 AI 配图」+「🎬 测试 AI 合成视频」+「📄 测试导出」三个独立按钮 | image / video / export |
| 2 | ✎ 录入(Compose)| 10 场景 × 10 主题 × 5 预设 三层 chip + 原文/意图输入 + demo 兜底 | 走 `agnes-3.0-flash` |
| 3 | ◇ 工坊(Studio)| 8 风格 chip · 7 AI 助手 chip · 内容正文编辑器 · 「重试/撤销/快照」 | text |
| 4 | ▤ 计划(Plan)| 条目增/删/上/下/edit · 真正 1-6 字段 TextInput 编辑 · AI 配图 · AI 合成视频(异步轮询) | image + video |
| 5 | ⚙ 设置(Settings)| API key 输入 · 「测试连接」(1 token ping) · 主题 · 配额 | — |

入口出口:Plan 屏顶部「📤 导出 →」进入 Export(8 格式 chip:Markdown / 公众号 / Notion / SRT / 制作包 / Marp / markmap / 提纲)。

## 三个 AI 模型

| 模型 | 用法 | 协议 |
|---|---|---|
| `agnes-3.0-flash` | 文 —— 7 panel + 10 scenario schema,JSON 强校验 | OpenAI `chat.completions` |
| `agnes-image-2.5-flash` | 图 —— Plan 屏 AI 配图 + Home 测试按钮 | OpenAI `images/generations`(`extra_body.response_format:"url"`)|
| `agnes-video-2.5` | 视频 —— Plan 屏 AI 合成视频 + 异步轮询 | OpenAI Videos `POST /v1/videos` + `GET /agnesapi?video_id=…` |

服务端:`https://apihub.agnes-ai.com/v1`(可由 `OCTOSTUDIO_AGNES_BASE_URL` 覆盖)。
鉴权:用户填进 Settings 屏的 Bearer API key(v0.6 明文保存,v0.7 加密)。

## 5 屏截图

| 屏 | 文件 |
|---|---|
| Home | [native/screenshots/v060-home.png](native/screenshots/v060-home.png) |
| Compose | [native/screenshots/v060-compose.png](native/screenshots/v060-compose.png) |
| Studio | [native/screenshots/v060-studio.png](native/screenshots/v060-studio.png) |
| Plan | [native/screenshots/v060-plan.png](native/screenshots/v060-plan.png) |
| Export | [native/screenshots/v060-export.png](native/screenshots/v060-export.png) |
| Settings | [native/screenshots/v060-settings.png](native/screenshots/v060-settings.png) |

## 跑起来

```bash
cargo build --release -p octostudio-app
./target/release/octostudio --port 8141
```

完整启动 + 调试 + Studio 模式说明 → **[quick_start.md](quick_start.md)**。

## 工作空间

8 crate,职责边界清晰,共享 `octostudio-core` 类型:

```
crates/
├── octostudio-core      # types: Work / PlanItem / Composition / Screen / PlanKind
├── octostudio-theme     # tokens: 16 color + 7-step type scale
├── octostudio-storage   # JSON IO: works.json / config.json / usage.json + splash 迁移
├── octostudio-ai        # Agnes 文/图/视频 3 客户端 + 7 panel helper + 10 scenario
├── octostudio-export    # 8 format renderers: Markdown/公众号/Notion/SRT/制作包/Marp/markmap/提纲
├── octostudio-render    # AppState: 35+ field,屏幕渲染函数
├── octostudio-demo      # 2 个首启动 demo works
└── octostudio-app       # binary: main, app.rs 的 script_mod! 块含 5 屏 + TabBar
```

`makepad-widgets` 解析到 `../makepad/widgets`(同机相邻目录,path dep)。

## 测试

```bash
cargo test --workspace          # 31 个用例,~3s
```

## 历史说明

v0.6 起从 Splash 脚本完全迁移到 Makepad Rust native。Splash 时代相关文件
(`bundle/`、`build/` 提交物、`BRIEF.md`、`ROADMAP.md`、`INTEGRATION.md`、
`REVIEW-ANSWERS.md`、`ISSUES.md`、`assets/` 旧宣传物料、`promo/` 旧宣传片工程)
已在 v0.6 分支清理。

## License

Apache-2.0,见 [LICENSE](LICENSE)。
