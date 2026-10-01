# OctoStudio

> **本地创作工作台 · 设备 AI 二创(在 Rinx 中可用)**

OctoStudio 是一个 OctoSense **脚本应用** — 整个应用就是 `bundle/main.splash` 一个文件,被 Splash VM 解释执行,运行在 `card-host` 隔离沙箱里。它提供作品库 → 录入 → 创作 → 导出 四步流程;在 Rinx 中可调用设备 AI 做改写 / 翻译 / 总结 / 评论,在独立 `card-host` 中助手返回 `no service answers`,UI 仍可手动创作并保存。

## 这是什么形态的应用?

OctoSense 脚本应用是一个小巧、隔离运行的应用包,**只有 `bundle/` 会被提交**:

```text
octostudio/
├── README.md                ← 你正在读
├── BRIEF.md                 设计意图
├── REVIEW-ANSWERS.md        答 hub scan 7 个审核问题
├── build/review.json        hub scan packet
├── bundle/                  ★ THE SUBMISSION
│   ├── manifest.json        id、name、version、capabilities、hosts、integrity
│   ├── listing.json         商店展示的元数据
│   ├── main.splash          程序本体(298 行,Splash 脚本)
│   ├── assets/icon.svg      列表上的图标
│   └── screenshots/         真实截图(01-home … 05-ai-error)
├── AGENTS.md  CLAUDE.md  GEMINI.md   Agent 规则(从模板复制,不提交)
├── .gitignore               忽略 build/ / target/ / .local-state/ / *.key
└── .local-state/            card-host 的 jail + log(本地调试用,不提交)
```

**`main.splash` 本身就是程序**。它不是配置文件,没有 .py / .rs / .ts 配套。整个应用是一个被 Splash VM 解释执行的脚本应用,顶层 `let` 是状态、`fn` 是函数,然后一个根 View。

---

## 前置条件

按 [`OctoScript-App-Design-Flow/README.zh-CN.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/README.zh-CN.md) 的快速上手:

| 工具 | 版本 | 备注 |
| --- | --- | --- |
| macOS(Apple silicon 已验证) | 14+ | Windows / Linux 未验证 |
| Rust(stable via rustup) | `~/.cargo/bin` 加入 PATH | 用来构建 `hub` 与 `card-host` |
| Python | 3.9+ | macOS 自带的 `/usr/bin/python3` 即可 |
| 图形会话 | — | card-host 会开一个 412×892 点的窗口 |

工作区布局 — 五个仓库并排放置:

```text
<workspace>/
├── OctoScript-App-Design-Flow/   # 工具链入口
├── OctoSense-App-Hub/            # hub、card-host 的 Cargo workspace
├── makepad/                      # 框架
├── octoscript-makepad/           # Splash + widgets
└── octoscript/                   # 语言运行时
```

本项目 (`octostudio/`) 是第六个目录,在工作区外面随便放哪里都行,不在依赖图中。

---

## 一次性构建工具链

进入工作区,从 `OctoSense-App-Hub` 编译两个二进制(总产物约 1 GB,首次约 1 分钟):

```sh
cd <workspace>
git clone https://github.com/OctoSense-org/OctoScript-App-Design-Flow.git
git clone https://github.com/OctoSense-org/OctoSense-App-Hub.git
cd OctoScript-App-Design-Flow && python3 tools/setup-native.py

# 这一步会拉 makepad / octoscript / octoscript-makepad 三个仓库并锁定到
# native-runtime.lock.json 指定的 commit。可重复跑、幂等。
(cd ../OctoSense-App-Hub && cargo build --release -p octosense-card-host -p octosense-app-hub)

tools/octo doctor    # 应该全部 [ok]
```

> 产物路径:`$CARGO_TARGET_DIR/release/{hub,card-host}`,默认会找到。如果你的 CARGO_TARGET_DIR 不一样,设置 `OCTO_HUB=/path/to/hub` / `OCTO_CARD_HOST=/path/to/card-host` 即可。

---

## 看到页面 — 三种模式

### 模式 1:Agent / 脚本驱动(无头,推荐)

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo run /Users/lijing/CodeProjects/octostudio/bundle \
    --port 8142 --hidden --detach

# 输出示例:
#   card-host: octostudio 0.1.0 admitted — capabilities {…}
#   ready: first frame drawn
#   pid 81880  log /Users/lijing/CodeProjects/octostudio/.local-state/card-host.log
```

`--hidden` 让 card-host 不开窗口、不抢焦点,远程控制桥 (`/snap`, `/click`, `/g` 截图) 照常工作。**你可以同时跑多个应用、多个端口,互不抢屏幕。**

远程控制桥路由(均为 GET,坐标是窗口 CSS 点,y 向下):

| 路由 | 作用 |
| --- | --- |
| `/snap?q=…` | 组件矩形 + 文本,例如 `/snap?q=AI` |
| `/d` | 整棵组件树的文本形式 |
| `/click?x=&y=&wait=1` | 一次真实点击(坐标是 CSS 点,824×1606 的截图对应 412×803) |
| `/t?t=TEXT&wait=1` | 向获得焦点的输入框输入文字 |
| `/k?k=down&c=ReturnKey` | 一个按键事件 |
| `/log?n=50` | 最近的日志行 |
| `/g?raw=1` | 窗口的 PNG(就是 `tools/octo shot` 保存的内容) |
| `/quit`(或 `/gq`) | 退出 — **最后一定要调用** |

完成后:

```sh
curl -s 127.0.0.1:8142/quit
```

### 模式 2:肉眼可见窗口(交互调试)

去掉 `--hidden`,card-host 会弹一个 412×892 点的窗口,你直接点。

```sh
tools/octo run /Users/lijing/CodeProjects/octostudio/bundle --port 8142
# 窗口出现,你能看见、能点
# Ctrl+C 退出
```

`--detach` 在 macOS 上仍会让窗口出现,但 fork 到后台不会卡你的 shell。

### 模式 3:在 OctoSense Shell 里看(发布前的真实路径)

`card-host` 是参考实现。真实运行环境是 OctoSense 桌面端 Shell 和手机 Home 端,通过 App Hub 的 Card runner 执行同一套 manifest 策略。系统应用从 OctoSense 的 `apps/` 打包进 Shell 构建;商店应用从 App Hub 商店、依据签名目录安装。

要在发布前在桌面 Shell 中试用:

1. 用一次性信任锚把 OctoStudio 发布到本地目录
2. 把桌面端 Shell 的 `OCTOSENSE_HUB` / `OCTOSENSE_HUB_ANCHOR` 指向该目录
3. 在 App Hub 里安装并打开

详细步骤见 [`docs/PUBLISHING.md §4`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/PUBLISHING.md#4-rehearse-the-store-path-locally)。

手机上:目前**不能**把任意应用包侧载到普通 OctoSense 手机。手机商店读取内置的 hub,只信任编译进构建的信任锚;`OCTOSENSE_HUB` / `OCTOSENSE_HUB_ANCHOR` 是环境变量,Android 启动器不会设置。`card-host` 的远程控制桥在 Android 上被编译移除。

---

## 开发循环

### 改完一次,看到的下一次启动要重启

Splash 脚本是**解释执行**的,所以不需要 cargo 编译。但 card-host 启动时一次性 eval,所以你改了 `bundle/main.splash` 必须重启进程才能看到新版本:

```sh
# 1. 改 bundle/main.splash(任何你常用的编辑器)
$EDITOR /Users/lijing/CodeProjects/octostudio/bundle/main.splash

# 2. 停掉旧实例
pkill -f card-host
# 或:curl -s http://127.0.0.1:8142/quit

# 3. 重启
tools/octo run /Users/lijing/CodeProjects/octostudio/bundle --port 8142 --hidden --detach

# 4. 立刻看一下首屏对不对
tools/octo shot 8142 /tmp/octo_dev.png
open /tmp/octo_dev.png
```

### 看一眼有几种 view

```sh
# 列出所有可见的组件和它们的矩形
curl -s "127.0.0.1:8142/snap" | python3 -m json.tool | head -40

# 找特定名字的按钮
curl -s "127.0.0.1:8142/snap?q=新建" | python3 -m json.tool

# 整棵组件树的文本
curl -s "127.0.0.1:8142/d" | head -30
```

### 模拟点击

```sh
# 点 home 页的 "新建作品" 按钮(右上角,坐标 x=327 y=166 是 CSS 点)
curl -s "127.0.0.1:8142/click?x=327&y=166&wait=1"

# 输入文本(需先 focus 输入框 — 上面那个 click 已经 focus 了)
curl -s --get "http://127.0.0.1:8142/t" --data-urlencode "t=AI 写作并不是…"
```

完整截图捕获:

```sh
tools/octo shot 8142 /Users/lijing/CodeProjects/octostudio/bundle/screenshots/01-home.png
```

> 截图是在 **App 自身渲染管线**里出的图(应用自己把 PNG 写到磁盘),即使在 `--hidden` 下也能拿到完整画面。

### 看日志(出错时第一件事)

```sh
tail -30 /Users/lijing/CodeProjects/octostudio/.local-state/card-host.log
# 找 [E] splash: line:col - … 错误行
# [SPLASH] eval: NN bytes 后面是编译错误
```

如果你看到 `method X not found on object` — 那是 Splash 脚本里调用了不存在的 API。检查名字、对照 [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md)。

### 调试脚本:`sys.time_ms()` 之类的坑

Splash isolate **剥离**了 `mod.fs` / `mod.run` / `mod.res` / `mod.cx.quit`,这几个全局名在脚本里都是 `nil`(见 `widgets/src/widget_async.rs:562`)。如果你看到 `variable X not found`,多半是这几个之一。

时间戳用 `std::time::SystemTime` 也拿不到 — Splash 没有暴露这个 API。OctoStudio 里用一个 `seq` 单调递增计数器代替。随机数同理,自己做 seed。

---

## 跑 `octo check`(准入检查)

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo check /Users/lijing/CodeProjects/octostudio/bundle
```

期望输出:

```text
octostudio 0.1.0 — PASSED
  [warning] publisher-signature: unsigned: accountability rests on the hub alone
  grants: capabilities {"images", "octos.session.open", "octos.turn.start", "storage", "web"}, hosts {}, storage 16777216 bytes, agent none
octo: note: listing.json still holds template placeholders (example.com); the gate accepts them, a reviewer will not.
```

`check` 先 `hub stamp` 写入 `bundle_blake3` 到 `manifest.json`,再 `hub check --allow-unsigned`。如果拒绝,看具体哪个权限或 listing 字段被拒。

---

## 写新功能

1. 打开 `bundle/main.splash`
2. 找对应的 `let` 状态、`fn` 函数、根 View 里的 tab/screen
3. 编辑、保存
4. `pkill -f card-host && tools/octo run …`
5. `tools/octo shot` 看效果
6. 改完 → `tools/octo check` 跑过 → 重新生成截图 → commit

Splash 语言基础:

- `let x = expr` 顶层 `let` 是模块状态(跨视图/回调共享)
- `let x = 0` `x += 1` 标量读写
- `let arr = []` `arr.push(value)` `arr.len()` 数组
- `for i in 4 { … }` `for w in works { … }` 循环
- `if cond { … } else { … }` 条件
- `fn name(args){ … return expr }` 函数
- 闭包 `|| { … }` 或 `|| expr`(单表达式)
- 对象字面量 `{key1: value1 key2: value2}`
- 字符串拼接 `"a-" + 1.to_string()`
- 注释 `// …` 行,`/* … */` 块

UI 组件:

- `Label{text: …}`
- `ButtonFlat{text: … on_click: || …}` / `GestureView{on_tap: || …}`
- `TextInput{empty_text: …}`(占位),`ui.my_input.text()` 读,`set_text(…)` 写
- `View{flow: Down/Right/Overlay padding: … spacing: … width: Fill height: Fill/Fit}`
- `Image{src: … fit: ImageFit.CropToFill}`
- `ScrollYView{…}` 滚动容器
- `tabs.on_render: || { … }` 闭包形式的动态生成,**记得显式 `ui.<name>.render()` 触发重绘**
- 切屏:`ui.a.set_visible(false); ui.b.set_visible(true)`(`Overlay` 父容器里)
- 模态:`show_dialog(modalId)`

API 全部见 [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md);Gotchas 部分节省 80% 的调试时间。

---

## 发布到 App Hub

`tools/octo package-help` 会输出完整清单。简要步骤(全是人手):

```sh
# 1. 修掉 listing.json 里的模板占位
#    publisher.name: "AmosLi (TODO 替换为发布者名称)" → 你的名字
#    publisher.support / privacy_policy_url: "https://example.com/..." → 真实 URL

# 2. 生成签名密钥(仓库外保存,只跑一次)
hub keygen /secure/path/publisher.key

# 3. 签名
hub sign-manifest /Users/lijing/CodeProjects/octostudio/bundle \
  --key /secure/path/publisher.key --key-id amosgeek

# 4. 带密钥重检
hub check /Users/lijing/CodeProjects/octostudio/bundle \
  --publisher-key amosgeek=$(hub pubkey /secure/path/publisher.key)

# 5. 在 GitHub 上打 tag
git tag octostudio-0.1.0 && git push origin octostudio-0.1.0

# 6. 在 OctoSense-App-Hub 开 issue
#    标题:Submit octostudio 0.1.0
#    附:tag、commit、bundle 路径、publisher 公钥、check 输出、REVIEW-ANSWERS.md
```

签名之后任何修改都要重新 stamp + 签名。

---

## 故障排除

| 现象 | 原因 | 修复 |
| --- | --- | --- |
| `port 8142 already in use` | 上一个 card-host 没退 | `pkill -f card-host` 或 `curl -s 127.0.0.1:8142/quit` |
| `[E] splash: line:col - method X not found` | 调了不存在的 API | 查 `docs/SCRIPT-API.md` |
| `[E] splash: line:col - variable X not found` | 用了 `fs / run / res / cx.quit` 之一 | 这些在 isolate 里被剥成 nil,改用自己的实现 |
| 首屏出来后一直是 loading | `start_timeout(0.05, || boot())` 没跑,或 boot 报错 | 看 `card-host.log` 的 `[SPLASH] eval:` 后面 |
| `ai` 按钮点了之后状态不变 | `host.request("octos.turn.start", …)` 返回 `{ok: false, error: …}` | card-host 不提供宿主服务 — 这是 README §"应用中的 AI" 描述的降级,UI 应给出"no service answers" |
| 截图里输入框是空的 | 输入后没等渲染就 shot 了 | 加 `--settle 2`(默认 2 秒) |
| 改了 main.splash 没生效 | card-host 还在跑旧版本 | `pkill -f card-host` 再重启 |
| `hub check` 报 `refused listing: …` | listing 引用了不存在的文件 | 真实截图放进 `bundle/screenshots/` |

---

## 相关链接

- [`OctoScript-App-Design-Flow`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow) — 工具链与文档
- [`OctoSense-App-Hub`](https://github.com/OctoSense-org/OctoSense-App-Hub) — `hub` / `card-host` / 商店
- [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md) — Splash 语言 + 全部 API
- [`docs/CAPABILITIES.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/CAPABILITIES.md) — 每种权限解锁什么、用户看到什么
- [`docs/AI-SERVICES.zh-CN.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/AI-SERVICES.zh-CN.md) — `octos.*` 当前能做什么、规划
- [`docs/PUBLISHING.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/PUBLISHING.md) — 发布到 App Hub

## 许可证

Apache-2.0。