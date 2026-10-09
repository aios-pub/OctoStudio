# OctoStudio v0.6 · Quick Start

> **原生 Makepad Rust 桌面应用 · 3 个真 Agnes AI · 全 OpenAI 兼容**
> Splash / card-host / OctoSense / Rinx 已脱钩。本文所有命令与参数都对照源码验证过。

---

## 1. 环境前置

| 项 | 要求 |
|---|---|
| OS | macOS arm64(Apple Silicon)或 Linux x86_64 |
| Rust | stable toolchain(`cargo` 可用) |
| Makepad | 本地 checkout 在 `/Volumes/PSSD/CodeProjects/makepad`(`Cargo.toml` 用 path dep 指向 `../makepad/widgets`) |
| 网络 | 调 AI 时需可达 `apihub.agnes-ai.com`;纯 UI 调试不需要 |
| API key | 在 [agnes-ai.com](https://www.agnes-ai.com) 申请,首启后在设置屏填入 |

无 tokio / async-std / 系统服务;单进程 native event loop。

---

## 2. 构建 + 运行(30 秒)

```bash
cd /Volumes/PSSD/CodeProjects/octostudio

# release 编译(首跑 1-2 分钟,增量数秒)
cargo build --release -p octostudio-app

# 跑:1280x800 窗口(尺寸硬编码在 crates/octostudio-app/src/app.rs:844)
./target/release/octostudio
```

窗口打开即就绪:左侧飞书风导航栏(广场/录入/工坊/计划/设置)+ 右侧内容分屏(可滚动)。没有 API key 时
顶部会显示「⚠ 需要 API key → 设置」banner,非 AI 功能照常可用。

**窗口尺寸**:没有 `--size` 参数。要改尺寸,编辑
`crates/octostudio-app/src/app.rs` 里的 `window.inner_size: vec2(1280, 800)` 后重编。

---

## 3. 调试模式:`--remote` HTTP 控制桥

makepad 在**每个 app** 里内置一个 localhost HTTP 控制面(`platform/src/remote.rs`),
用 `--remote` 参数或 `MAKEPAD_REMOTE=1` 环境变量打开:

```bash
# 三种等价开法
./target/release/octostudio --remote &              # 随机端口,stdout 打印实际端口
./target/release/octostudio --remote=8141 &         # 固定 8141
./target/release/octostudio --remote=127.0.0.1:8141 &
MAKEPAD_REMOTE=1 ./target/release/octostudio &      # 环境变量写法
```

启动后 stdout 会打一行(记下端口):

```
[makepad-remote] listening on 127.0.0.1:8141 pid=12345 app=octostudio grabs=...
```

### 常用端点(全部 GET;`curl '127.0.0.1:8141/'` 看完整表)

| 端点 | 作用 |
|---|---|
| `/s` | 窗口列表 + 尺寸 + 位置(JSON) |
| `/g?raw=1` | 抓整窗 PNG(`raw=1` 直接回图片字节;不带则落盘并返回路径) |
| `/snap?q=` | **可点击坐标清单**:所有可见 widget 的 id / 类型 / 文本 / 矩形;`q=` 按子串过滤 |
| `/click?x=&y=` | 真实事件路径注入点击(等效真鼠标) |
| `/t?t=文字` | 向焦点控件注入文本输入 |
| `/k?c=KeyA` | 原始按键(down/up;支持 `Escape` `ReturnKey` `Tab` `Backspace` 等) |
| `/d` | 整棵 widget 树(缩进文本,id + 类型 + x y w h) |
| `/log?n=50` | 尾部应用日志 |
| `/gq` | 抓全部窗口 PNG 然后**优雅退出**(批量验证收尾用这个) |
| `/quit` | 优雅退出 |

**调试套路**:`/snap` 找到目标按钮的矩形 → 取中心点 → `/click?x=&y=&wait=1`
(`wait=1` 等下一帧画完才回)→ `/g?raw=1 -o shot.png` 验证。输入进文本框前
先 click 聚焦,再 `/t?t=`。

---

## 4. Makepad Studio 模式

Makepad 的官方 IDE 叫 **Director**,在 makepad checkout 里跑:

```bash
cd /Volumes/PSSD/CodeProjects/makepad
cargo run -p makepad-director --release
```

Director 提供:项目树、实时代码→UI 迭代、widget inspect、AI 自动化控制 UI。

**本项目与 Director 的两种配合方式**:

1. **自跑 + HTTP 桥(推荐,本仓库全部验证过这种方式)**——
   按第 3 节 `--remote=8141` 自行启动 octostudio,用 curl 驱动。
   remote 桥的输入走 `Cx::dispatch_studio_msg`,**与 Studio 内部用的是同一条
   事件路径**,所以 curl 驱动 = Studio 驱动,行为完全一致。
2. **Director 里打开本项目**——Director 能打开本地 Rust workspace 作为 project;
   octostudio 是标准 cargo workspace,可在 Director 内浏览/编辑
   `crates/octostudio-app/src/app.rs` 并触发编译运行。

> 没有单独的「studio build」开关——desktop binary 天生就能被 Studio/remote 附着。

---

## 5. 测试

```bash
cargo test --workspace          # 全部 31 用例,~3s
cargo test -p octostudio-ai     # 文/图/视频 3 客户端
cargo test -p octostudio-export # 8 格式渲染器(srt 时间码 roundtrip)
cargo test -p octostudio-storage# works/config/usage + splash 迁移
```

覆盖:`octostudio-ai` 7 · `octostudio-core` 5 · `octostudio-export` 9 ·
`octostudio-storage` 4 · `octostudio-render` 1 · `octostudio-theme` 2 ·
`octostudio-demo` 3。

---

## 6. 真实 AI 跑一遍(带 key,5 步)

```bash
./target/release/octostudio --remote=8141 &

# 1. 找到设置 tab 的可点击坐标(底部最右)
curl -s '127.0.0.1:8141/snap?q=settings' 

# 2. 点进去(坐标来自上一步矩形中心)
curl -s '127.0.0.1:8141/click?x=1250&y=780&wait=1'

# 3. 点 API key 输入框聚焦,注入 key
curl -s '127.0.0.1:8141/click?x=<输入框中心>&wait=1'
curl -s '127.0.0.1:8141/t?t=ag-你的key'

# 4. 点「保存」→ 点「测试连接」(发一次 1-token chat)
curl -s '127.0.0.1:8141/click?x=<保存按钮中心>&wait=1'
curl -s '127.0.0.1:8141/click?x=<测试连接中心>&wait=1'

# 5. 抓图确认状态栏显示连接成功
curl -s '127.0.0.1:8141/g?raw=1' -o /tmp/settings-ok.png
curl -s '127.0.0.1:8141/quit'
```

没有 key 也行:Home 的「🎨 测试 AI 配图」「🎬 测试 AI 合成视频」「📄 测试导出」
自动降级到 `octostudio-demo` 预置数据,状态栏标注演示来源。

**base URL 覆盖**(测试指向别的 OpenAI 兼容服务):

```bash
OCTOSTUDIO_AGNES_BASE_URL=http://127.0.0.1:9999/v1 ./target/release/octostudio
```

---

## 7. 视频生成的异步轮询(排查时看)

Agnes 视频是异步任务,链路:

```
Plan 屏「🎬 AI 合成视频」
  → POST /v1/videos               (~立即返回 video_id)
  → Cx timeout 每 2s 轮询
     GET /agnesapi?video_id=<ID>&model_name=agnes-video-2.5
  → queued → running(progress 0-100)→ completed(url) / failed(错误)
  → 70s 看门狗(handle_watchdog_timer_fire)兜底超时,防 UI 假死
```

---

## 8. 数据落在哪

| 文件 | 路径 |
|---|---|
| `works.json` | macOS `~/Library/Application Support/octostudio/`;Linux `~/.local/share/octostudio/` |
| `config.json` | 同目录(API key 明文,v0.6 简化) |
| `usage.json` | 同目录(每日 token 聚合) |
| splash 旧数据 | 首启自动迁移 `~/.octosense/octostudio/works.json` 与 `<bundle>/.local-state/works.json` |

路径解析在 `crates/octostudio-storage/src/paths.rs`(`dirs::data_dir()`)。

---

## 9. 常见问题

| 现象 | 原因 / 修法 |
|---|---|
| 编译报 makepad 找不到 | `Cargo.toml` 的 path dep 指向 `../makepad/widgets`——确认同级目录有 makepad checkout |
| `--remote` 没打印 listening 行 | 参数写法错;必须 `--remote` / `--remote=PORT` / `MAKEPAD_REMOTE=1`,**没有 `--port`** |
| curl 无响应 | 先看 stdout 里实际绑定端口(不带 `=PORT` 时是随机端口) |
| AI 转圈不出 | 看状态栏:`No API key` → 设置填;`connection refused` → 网络到 `apihub.agnes-ai.com` 不通;>70s 会被看门狗掐断 |
| 改了 `item_fields` UI 不变 | `crates/octostudio-core/src/fields.rs` 改完要重编 binary(`script_mod!` 是编译期展开) |
| 想全屏/手机尺寸 | 改 `app.rs:844` 的 `window.inner_size` 后重编(无 CLI 参数) |
