# OctoStudio

> **言出法随 · 意图即应用 — 一句话,就是一篇文**

<!-- 视频介绍(60s @ 30fps,18 景精确 60s · v0.4.3 全品类:10 主题 / 5 视频预设 / M3 内容管理 / AI 助手 7 项 / 3 宿主真实运行)
     GitHub README <video> 标签被 strip → 动画 GIF 作内嵌预览;
     点击 GIF 跳转 B 站播放页(带音频/弹幕/高清);GitHub blob 页与 raw 下载作备选 -->
<p align="center">
  <a href="https://www.bilibili.com/video/BV1MZHW6MEWZ/">
    <img src="assets/promo-v10-preview.gif" alt="▶️ 点击观看视频介绍(哔哩哔哩)" width="960">
  </a>
</p>

<p align="center">
  <sub>▶️ <a href="https://www.bilibili.com/video/BV1MZHW6MEWZ/">哔哩哔哩观看【OctoStudio 功能介绍】</a>
  · <a href="https://github.com/aios-pub/OctoStudio/blob/main/assets/promo-v10.mp4">GitHub 播放 1080p</a>
  · ⬇️ <a href="https://github.com/aios-pub/OctoStudio/raw/main/assets/promo-v10.mp4">下载 MP4 (14.9 MB)</a>
  · 18 景精确 60.000s @ 30fps · 源工程 <a href="promo/">promo/</a> (fframes · Skia · Metal)</sub>
</p>

一句话,就是一篇文。

OctoStudio 让创作者的意图直接成为产物 — **言出法随,意图即应用**。跑在 [OctoSense](https://github.com/OctoSense-org) 设备上的脚本应用,整个 `bundle/` 就是一个 `main.splash` 文件,在隔离沙箱里被 Splash VM 解释执行。面向中文内容创作者(公众号作者 / Markdown 写作者 / 视频脚本写手):用一句话或一段原文,生成可发布到多个平台的内容,作品全部存在你设备本地,AI 用的是你设备上的模型而非云端 API。

`v0.4.3` · 3017 行 Splash · 5 屏 · 22 张真实截图 · Apache-2.0

---

## 这是什么

OctoStudio 是给 **中文内容创作者** 用的本地优先、AI 增强的创作工作台。它把"从一段原文 / 一句话意图 → 可发布到多平台"这条链做成一键流程,跑在 Rinx / OctoSense 设备上 — 作品存设备沙箱,跨设备不离开你的控制,调用的是用户设备上的 AI(密钥由 AI providers 系统应用托管)。

## 痛点 — 创作者面对的真问题

1. **多平台分发繁琐**:同一篇文章要粘到公众号后台、Markdown 编辑器、Notion,每次都重新调格式
2. **AI 改写生硬**:通用模型改出来"风格统一",但跟你自己的语气对不上,改完还得手动润色
3. **视频脚本难起手**:从一句话想法到分镜表,中间缺一座桥
4. **每次都得手写**:参考资料管理、版本管理、风格管理都靠脑子记,工具散在剪贴板 / 网盘 / 浏览器收藏夹

OctoStudio 把这四件事压缩到一台设备上的一个 app 里。

## 意境 — 言出法随,意图即应用

OctoStudio 的两条产品意境,贯穿所有屏幕、所有交互、所有降级路径。它们不是营销话术,而是驱动每行代码的底层信条:

### 言出法随

> **说出口的话,就是可发布的法(范式)。**

在传统工具里,创作者要先想好"用哪个模板、哪种风格、几号字体",再点十几个按钮才能产出一篇文。OctoStudio 反过来:**用户的输入即产物**。说一句"夏日海边慢生活,温暖散文风",图文 plan 立即落地;说一句"3 分钟产品发布会开场,极简风",视频分镜立即落地。说出口的话不打折、不被工具改写、不需要用户再做"翻译"。

### 意图即应用

> **用户的意图本身就是应用;不需要"配置应用"这一步。**

意图驱动一切:Compose 屏从输入意图 → `model.complete` 按 schema 出 plan → Export 屏多格式呈现,中间没有"打开模板"或"切换工作流"这类配置。**意图是输入,产物是输出,中间只有 AI 的创造性填充**。

### 两境相生

- **"言出法随"** 讲的是用户感受 — 说出口的话立刻成为可发布的产物,中间没有摩擦。
- **"意图即应用"** 讲的是产品形态 — 应用跟着意图走,意图是唯一的输入参数。
- 两条意境同时成立,OctoStudio 才完整:少一条就掉进"AI 玩具"(只生成不可用)或"传统工具"(只能配置不能意图)的旧范式里。

## 它做什么

一个 **场景广场**,十个创作入口,全部"一句话进,结构化出,可编辑,多格式出":

#### 能力在三种宿主下的实际表现(2026-10-04 实测)

OctoStudio 的所有 9 场景(原文二创 / 图文 / 视频分镜 / PPT / 拆解视频 / 标题工坊 / 小红书 / 口播 / 思维导图 / 金句语录)和工坊 8 风格二创,都依赖宿主实际接入的 `model` 与 `octos.turn.start` 服务。下面是与本机 `host.has()` 报告无关的实测结果:

| 宿主 | `model` | `octos.turn.start` | 9 场景生成 | 原文 8 风格二创 | 实际行为 |
|---|---|---|---|---|---|
| **OctoSense 桌面壳**(hosted) | ✅ | ✅ | ✅ | ✅ | AI 由本机 octos 内核 + 配 provider 真实返回 |
| **Rinx 小程序**(hosted 模式) | ❌ | ✅ | ⚠️ 部分 | ⚠️ 部分 | 原文二创可走,场景生成(model)降级;[Rinx#63](https://github.com/hagency-org/Rinx/issues/63) 记录 mini-app octos 异步回复缺口 |
| **card-host CLI** | ❌ | ❌ | ❌ | ❌ | 能力仅在 manifest 声明,无服务实际连接;全部走演示内容 |

`host.has("model")` / `host.has("octos.turn.start")` 在 card-host CLI 下会基于 manifest 声明返回 true(早期版本会显示"已就绪"),**但实际不连通**——这是 card-host 的设计,不是本应用 bug。v0.3.3 起在 card-host 下状态栏会显式标记"AI 仅声明,未连接",并指向 OctoSense 桌面壳以获得真实 AI 能力(见 [INTEGRATION.md](INTEGRATION.md))。


| # | 能力 | 用到的平台能力 | 截图 |
|---|---|---|---|
| 1 | **原文二创 × 8 种风格** — 粘贴原文,改写 / 翻译 / 总结 / 评论 / 润色 / 续写 / 扩写 / 小红书体 | `octos.turn.start` | 视频 17s |
| 2 | **图文文章** — 一句话 → 标题+摘要+段落+每段英文生图 prompt,配 **AI 配图** | `model.complete` + `images` | 视频 21s |
| 3 | **视频分镜** — 一句话 → 4–6 镜(时长/景别/画面/配音/生视频 prompt)+ 风格预设 ×5 | `model.complete` + `images` | 视频 25s |
| 4 | **AI 合成建议** — 分镜之上二次生成转场/配乐/节奏/调色/字幕/输出参数 | `model.complete` | 视频 25s |
| 5 | **视频制作包** — SRT 字幕(时间轴由分镜确定性生成)+ 素材清单 + ffmpeg 参考命令,照做即成片 | 纯文本转换 | 视频 38s |
| 6 | **PPT 演示** — 一句话 → 5–10 页(要点/讲稿/配图 prompt),导出 **Marp** 粘到 marp.app 即放映 | `model.complete` + `images` | 视频 28s |
| 7 | **拆解视频** — 粘贴爆款文字稿 → 结构/钩子/镜头语言/金句/**可复用骨架**,骨架可一键存为新分镜意图 | `model.complete` | 视频 28s |
| 8 | **标题工坊** — 一句话或原文 → 8 个候选标题 + 风格标签 + 推荐指数 | `model.complete` | 视频 42s |
| 9 | **小红书笔记 / 口播稿 / 思维导图 / 金句语录** — 标题+正文+标签+配图 / 钩子+节拍+CTA / markmap 导图 / 场景化金句 | `model.complete` + `images` | 视频 17s |
| 10 | **文章主题样式 ×10** — 公众号深度/干货清单/情感散文/知乎科普/诗歌意象……注入生成 prompt,随作品保存 | 本地数据 | 视频 17s |
| 11 | **通用计划编辑器** — 任何场景的产物逐条 **编辑/上移/下移/删除/新增** | 纯本地 | 视频 25s |
| 12 | **多格式导出** — Markdown / 公众号 / Notion / Marp / SRT / 制作包 / markmap,按场景动态出现 | 纯文本转换 | 视频 38s |
| 13 | **本地作品库** — 分类图标/相对时间/长按删除,存设备沙箱;v0.2 数据自动迁移 | `storage` | 视频 10s |
| 14 | **降级完备** — 任何 AI 服务不可用 → 填入演示内容,仍可编辑保存,失败原因原样显示 | `host.has()` + 预置 demo | 视频 25s |
| 15 | **广场搜索** *(v0.3.5)* — 顶部搜索框,实时过滤作品标题/原文关键词(`on_change`) | 纯本地 | 视频 38s |
| 16 | **标签筛选** *(v0.3.5)* — 标签 chips 从已有作品聚合;生成时按场景/风格自动打标签;点 chip 过滤作品 | 纯本地 | 视频 38s |
| 17 | **AI 历史** *(v0.3.5)* — 每篇作品最近 3 条 `{prompt, result}`,卡片可展开查看 | 纯本地 | 视频 42s |
| 18 | **M3 尾巴** *(v0.4.0)* — 排序 chips(最新 / 最早 / 按场景)/ 多选批量删除 / 撤销栈(保存前快照 5 步)/ 一键全选+复制 导出文本 | `storage` | 视频 42s |
| 19 | **AI 助手 7 项** *(v0.4.0)* — 工坊屏 model 助手面板 — 自动起标题/关键词/摘要/风格迁移(4 风格)/中英对照/标题打分/模型预算;`class: "fast"` 节省配额 | `model` | 规划中 |
| 20 | **glance 卡片** *(v0.4.0)* — 启动时声明 `glance` capability;提供 `glance.publish` / `glance.withdraw` 助手,降级时按钮置灰 | `glance` | 规划中 |
| 21 | **宣传片 / 宣传图** *(规划中)* — 初赛路演与商店展示物料(片源工程在 `promo/` 迭代中) | 工具链 | 规划中 |

## AI 能力真实测试报告(v0.4.0)

> 测试日期:**2026-10-05**。测试方式:在 `bundle/main.splash` 加入临时 `probe_all()` 函数,在 card-host 下点击「运行 AI 能力探查」按钮一次性发出 14 个 `host.request` 调用,把每个返回的 `is_ok` / `error` 拼成多行日志显示在屏。
> 截图见视频 38s / 42s 区间

### 能力矩阵与现状

| 能力 | 期望 | card-host 实测 | 平台文档(2026-10-01)状态 | 已提 issue |
|---|---|---|---|---|
| `model.complete` | 文本生成,schema 严格 | ❌ `no service answers "model" on this device` | **可用**(Shell 中) | — |
| `model.budget` | 配额查询 | ❌ `no service answers "model" on this device` | **可用**(Shell 中) | — |
| `octos.session.open` | 开启会话 | ❌ `no service answers "octos" on this device` | **可用**(用户允许后 Shell 中) | — |
| `octos.turn.start` | 助手对话 | ❌ `no service answers "octos" on this device` | **可用** | — |
| `octos.session.history` | 历史 | ❌ `this app was not granted "octos", which "octos.session.history" needs` | **可用**(manifest 未声明) | — |
| `glance.publish` | 发布到 glance 屏 | ❌ `no service answers "glance" on this device` | **可用**(Shell 中) | — |
| `glance.list` | 列出 glance | ❌ `no service answers "glance" on this device` | **可用** | — |
| `glance.withdraw` | 撤回 glance | ❌ `no service answers "glance" on this device` | **可用** | — |
| `sys.digest` | L0 数据源 | ❌ `this app was not granted "sys", which "sys.digest" needs` | **可用**(新 runtime 中) | — |
| **`model.image`** | **文生图** | ❌ `no service answers "model" on this device` | **❌ 未发布** — 文档从未列名 | ✅ [OctoSense-App-Hub#85](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/85) |
| **`model.video`** | **文生视频** | ❌ `no service answers "model" on this device` | **❌ 未发布** | ✅ [OctoSense-App-Hub#86](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/86) |
| **`model.audio`** | **文字转语音** | ❌ `no service answers "model" on this device` | **❌ 未发布** | ✅ [OctoSense-App-Hub#87](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/87) |
| **`model.embeddings`** | **嵌入** | ❌ `no service answers "model" on this device` | **❌ 未发布** | ✅ [OctoSense-App-Hub#88](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/88) |
| **`octos.image`** | **文生图** | ❌ `this app was not granted "octos", which "octos.image" needs` | **❌ 未发布** | ✅ [OctoSense#332](https://github.com/OctoSense-org/OctoSense/issues/332) |
| **`octos.video`** | **文生视频** | ❌ `this app was not granted "octos", which "octos.video" needs` | **❌ 未发布** | ✅ [OctoSense#332](https://github.com/OctoSense-org/OctoSense/issues/332) |

### 真实宿主复测(本机从源码跑集成测试)

> 测试日期:**2026-10-05**(v0.4.2)。测试环境:`/Volumes/PSSD/CodeProjects/OctoSense` `main`(包含完整 Shell 与所有 host-service 仓);`python3 tools/setup.py --update --cargo` 完成 vendored makepad/octoscript-makepad 锁定 + patch 应用;`CARGO_TARGET_DIR=/Volumes/PSSD/dev/rust-target` 增量编译,**全部增量编译在 ~3-5 分钟内**(之前 dev artifacts 都已经预热)。

#### 来源级证据:OctoSense 源码里实际存在的服务名

```sh
$ grep -hoE '"model\.[a-z_.]+"' apps/ai-providers/host-service/src/ -r | sort -u
"model.complete"

$ grep -rn '"budget"' apps/ai-providers/host-service/src/complete/mod.rs | grep -v '//' | head
"budget" => reply.send(Ok(self.host.budget(&call.app_id).to_json())),     # complete/mod.rs:703

$ grep -hoE '"octos\.[a-z_.]+"' crates/ai-host/src/ -r | sort -u
"octos.render"          # 给 webview_render.rs 内部用,不在脚本应用 namespace
"octos.session.history"
"octos.session.open"
"octos.turn.interrupt"
"octos.turn.start"

$ grep -hoE '"glance\.[a-z_.]+"' crates/shell/src/glance.rs | sort -u
"glance.json"           # 文件名,非服务
"glance.list"
"glance.publish"
"glance.withdraw"

$ grep -rn "sys\.digest" --include="*.rs" crates/ | grep -v '//'
crates/shell/src/glance.rs:218:    for request in plan.requests.iter().filter(|r| r.helper == "sys.digest") {
crates/shell/src/glance.rs:222:        _ => return Err(format!("a card binds only its own app's digests: ...")),
# 仅作为 L0 卡片 helper 名出现;**无 host-service 入口**
```

#### 来源级证据:未实现的 `model.image`/`model.video` 等返回的字面错误

```rust
// crates/ai-host/src/contained.rs:321
other => return Err(format!("Unknown Octos service {other}")),
// 实测:  ask("com.example.reader", "octos.admin", json!({}), false).unwrap_err()
//      == "Unknown Octos service octos.admin"     (contained/tests.rs:311)

// crates/shell/src/glance.rs:606
other => Err(format!("glance has no method {other:?}")),
// 实测: dispatch service "glance.<unknown>"  → "glance has no method \"<name>\""
```

#### 实际跑出来的 `cargo test` 结果(无 mock、无 stub provider、无网络)

```
$ cargo test -p octosense-ai-host --lib
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
# 含 22 个 contained/ 集成测试,直接调用 contained::parse / dispatch / ask():
#   - contained_rejects_unknown_method (octos.admin → "Unknown Octos service octos.admin")
#   - contained_session_calls_map_to_context_ops
#   - contained_apps_get_only_the_octos_services_their_manifest_declares
#   - contained_rejects_unsupported_arguments
#   - contained_rejects_empty_or_oversized_text
#   - the_shell_prepares_a_consented_apps_peer_and_its_panel_shares_it
#   ... 等

$ cargo test -p octosense-llm-service --tests
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
# 含 apps/ai-providers/host-service/tests/complete.rs 的 19+ 集成测试:
#   - a_reply_failing_the_schema_is_retried_once_then_refused
#   - a_url_in_the_reply_is_refused_unless_the_app_allows_urls
#   - an_app_without_the_capability_is_refused_before_anything_else
#   - the_card_runner_gate_and_the_service_agree_on_the_model_capability
#   - the_budget_runs_out_and_says_so
#   - the_profile_is_the_provider_source_and_a_keyless_provider_is_skipped
#   - no_provider_is_named_as_such
#   - daily_calls_run_out_too
#   ... 等

$ cargo test -p octosense-shell --lib
test result: ok. 787 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
# 含 48 个 glance/ 集成测试 + 全部 shell 内部测试
#   - glance::tests::the_same_card_id_replaces_and_each_app_is_capped
#   - glance::tests::publishing_is_rate_limited_per_app
#   - glance::tests::the_feed_orders_by_priority_then_recency_and_caps
#   - glance_card::tests::the_ai_written_mark_reaches_the_card_window
#   ... 等
```

#### 能力真实矩阵(由源码 + 测试 + 探查共同确认)

| 能力 | 源码实现 | `cargo test` 实跑 | card-host probe | 用户在真实 Shell 中将得到 |
|---|---|---|---|---|
| `model.complete` | ✅ `apps/ai-providers/host-service/src/complete/mod.rs` | ✅ 19 个集成测试 PASSED | "no service answers \"model\" on this device"(card-host 设计) | 真实文本生成;模型失败/拒绝/schema 错误按 6 类返回(`capability`/`no_provider`/`rate`/`budget`/`bad_request`/`invalid_output`) |
| `model.budget` | ✅ `complete/mod.rs:703` | ✅ 由 `for method in ["model.complete","model.budget"]` 测试覆盖 | 同上 | 真实预算查询;`{budget}` 字段返回,无 model/provider/key 泄漏 |
| `octos.session.open / turn.start / turn.interrupt / session.history` | ✅ `crates/ai-host/src/contained.rs:318-321` | ✅ 22 个 contained/ 测试 PASSED | `no service answers "octos"`(首次)→ `unknown method` / `not granted`(未声明) | 首次调用需要用户允许 Agent;之后真实 peer,助手(壳/octos 内核)真实回答;`octos.admin` 等未知服务返回 `Unknown Octos service <name>` |
| `glance.publish / withdraw / list` | ✅ `crates/shell/src/glance.rs:601-608` | ✅ 48 个 glance 测试 PASSED | `no service answers "glance"` | L0/Splash 卡片真实发到屏;每应用速率限制;`glance.<other>` 返回 `glance has no method "<other>"` |
| `sys.digest` | ❌ 仅 glance.rs:218 作为 L0 helper 名解析出现,**无 host-service 入口** | ❌ 不在 ai-host/ 也不在 llm-service/ | `not granted "sys"`(manifest 未声明) | L0 卡片 helper 名;App Hub `#87` (sys.digest binding 还没合到 Shell);**当前不在 API 表面** |
| **`model.image`** | ❌ 不在源码(`grep` 零结果) | ❌ 无 | `no service answers "model"` | **不可用**;已提 [App-Hub#85](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/85) |
| **`model.video`** | ❌ 不在源码 | ❌ 无 | 同上 | **不可用**;[App-Hub#86](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/86) |
| **`model.audio`** | ❌ 不在源码 | ❌ 无 | 同上 | **不可用**;[App-Hub#87](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/87) |
| **`model.embeddings`** | ❌ 不在源码 | ❌ 无 | 同上 | **不可用**;[App-Hub#88](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/88) |
| **`octos.image`** | ❌ 不在源码 | ❌ 无 | `not granted "octos", which "octos.image" needs` | **不可用**;[OctoSense#332](https://github.com/OctoSense-org/OctoSense/issues/332) |
| **`octos.video`** | ❌ 不在源码 | ❌ 无 | `not granted "octos", which "octos.video" needs` | **不可用**;[OctoSense#332](https://github.com/OctoSense-org/OctoSense/issues/332) |
| `images` capability(URL loading) | ✅ `bundle/main.splash` 走 `http_resource()` | n/a | ✅ 配图直接加载,失败时降级为 prompt 文本 | ✅ 真实 |
| `storage` capability(本地沙箱) | ✅ `fs.read/write` 落 `works.json` | ✅ host 真实 | ✅ | ✅ |
| `web` capability(网络) | ✅ `http_resource(url)` | n/a | ✅ | ✅(但 `network.hosts=[]`,任何 url 都会被拒;OctoStudio 走 `images` 公开域) |

#### 结论(来源 + 测试 + 探查三处一致)

- **真实可用的能力** = `model.complete` + `model.budget` + `octos.{session.open, turn.start, turn.interrupt, session.history}` + `glance.{publish, withdraw, list}`,共 9 个,全部有源码实现 + 集成测试 PASSED。
- **OctoStudio 的所有 AI 功能都使用这 9 个能力**(M4 助手 7 项 → `model.complete` + `model.budget`;原文 8 风格二创 → `octos.session.open` + `octos.turn.start`;M5 glance → `glance.publish`/`glance.withdraw`)。
- **未实现的能力** = `model.image / model.video / model.audio / model.embeddings / octos.image / octos.video / octos.audio / octos.embeddings`,共 8 个,源码零实现,5 个 issue 已上报。
- **`sys.digest`** 名义上"已合入"(`OctoScript#40` + `OctoScript-Makepad#50`),但 OctoSense Shell 还没把它接到 host-service 表面(`OctoSense#87` 即将推出)。

### AI 提示词 / 场景模板 / 变量机制完备性核实(v0.4.3)

> 测试日期:**2026-10-05**(v0.4.3)。本节聚焦"AI 依赖"侧:prompt 模板、JSON Schema、变量注入、降级链路,能否真实产出可生产级内容。测试方法是把 `bundle/main.splash` 里**实际**发出去的 `host.request` args 序列化,直接走 `octosense-llm-service` 的真实 dispatch 路径(无 mock、无 stub provider、无网络),用假 provider 检验 host 接收的字段形状合法且 schema 校验通过。

#### 1. 场景 × 模板 × schema × 渲染器闭环(10 场景 × 7 导出格式)

| 场景 | task 模板 | schema 顶层字段 | 渲染器(`render_plan_as_text`) | 导出格式 |
|---|---|---|---|---|
| `image`(图文) | 把意图扩成 3-6 段,每段配英文生图 prompt + 主题语气 | `{title, summary, sections[3-6]{heading, body, image_prompt}}` | `rt_image(fmt)` | MD/公众号/Notion |
| `video`(分镜) | 把意图扩成 4-6 镜(含镜号/时长/景别/画面/配音/生视频 prompt)+ 视频风格预设注入 | `{title, logline, scenes[3-6]{id, duration_s, shot_type, description, voiceover, video_prompt}}` | `rt_video` + `srt_text` + `pack_text` | **MD/公众号/Notion/SRT/制作包** |
| `ppt`(演示) | 把意图扩成 5-10 页(封面/目录/内容/引言/结尾),每页 2-5 条要点 + 讲稿 + 配图 prompt + 主题语气 | `{deck_title, subtitle, slides[5-10]{kind, title, bullets[2-5], notes, visual_prompt}}` | `rt_ppt` + `marp_text` | **Marp/大纲/Notion** |
| `teardown`(拆解) | 拆用户提供文字稿,提取主线/钩子/结构/镜头/金句/复用骨架/要点 | `{video_title, one_liner, hook{pattern, why}, structure[3-6], shot_language[2-5], golden_quotes[1-5], reusable{angle, script_skeleton[3-6]}, takeaways[2-4]}` | `rt_rows` | MD/公众号/Notion |
| `titles`(标题工坊) | 给主题或原文生成 8 个候选,覆盖悬念/数字/对比/情感/干货 | `{titles[5-8]{text, style_tag, score 1-100}}` | `rt_titles` | MD/公众号 |
| `xhs`(小红书) | 标题带 emoji + 口语化正文 + 标签 + 3 张配图 prompt + 主题语气 | `{title ≤24, body ≤800, tags[3-6] ≤12字, image_prompts[3]}` | `rt_xhs` | MD/小红书正文/Notion |
| `script`(口播) | 30-60 秒口播:前 3 秒钩子 + 3-6 拍 + CTA + 总秒数 + 语气 + 主题语气 | `{hook ≤60, beats[3-6]{label, line}, cta ≤60, total_s 15-180, tone}` | `rt_rows` | MD/公众号/Notion |
| `mindmap`(思维导图) | 1 中心 + 3-6 分支 + 每支 2-5 子节点 | `{root, branches[3-6]{label, children[2-5]}}` | `rt_rows` + `markmap_text` | MD/**markmap**/公众号 |
| `quotes`(金句) | 6-10 条金句 + 适用场景 | `{quotes[6-10]{text, use_case}}` | `rt_rows` | MD/公众号 |
| `""`(原文二创) | 改写/翻译/总结/评论/润色 | (走 octos 二创) | `current_content` | — |

变量注入机制(源码可见):
- **主题(10 款)**:`theme_prompt()`(line 359-367)把 `current_theme` 的 `tone` 拼到 `task` 尾部,影响 image/ppt/xhs/script 共 4 个场景。
- **预设(5 款)**:`preset_line()`(line 369-377)把 `current_preset` 的 "电影感(电影级调色、稳定运镜、浅景深)" 注入视频 task + input。preset 直接影响生视频 prompt 的风格语。
- **场景输入**:每个场景独立的 `scenario_input()`(line 1080-1092)→ `scenario_task()`(line 1062-1078)→ `scenario_schema()`(line 1094-1234)→ `absorb_output()`(line 1236-1429)四件套,**路径一一对齐**:schema 的 required 字段在 absorb 里逐一赋值,absorb 不识别的字段会被 `additionalProperties: false` 拒收。
- **生成-编辑-保存闭环**:`ask_scenario()`(line 1806-1876)→ `load_demo_plan()`(line 1431-1494)兜底,失败时自动填入演示内容并跳转到计划编辑器,用户可在编辑后保存回 `works.json`。
- **风格工坊(8 项)**:`current_style` 切换"改写/口语化/学术/营销/润色/续写/扩写/小红书体",仅作用于原文二创,经 `octos.turn.start` 走壳内核的真实模型。

#### 2. 关键 Bug 修复(v0.4.3):M4 6 个 `model.complete` 调用的 args 形状

源码依据:`apps/ai-providers/host-service/src/complete/mod.rs:193-217` `Request::from_args` 明确拒收 `task/input/schema/class/allow_urls` 之外的所有键,且 `class` 必须在顶层(非 `output` 包装内);`tests/complete.rs:267-272` 已有断言:`system/max_tokens/model/provider` 等被拒,返回 `bad_request:` 错误。

v0.4.0-v0.4.2 的 6 个 M4 调用(`ai_gen_title` / `ai_extract_keywords` / `ai_summarize` / `ai_style_variants` / `ai_translate_zh_en` / `ai_score_title`)错误地传入了 `instructions`、`output` 包装、`temperature`、`max_tokens` 字段(均被 host 拒收),并把 `class` 嵌在 `output` 内导致缺省变 `fast` 而非显式指定。

修复(v0.4.3,`bundle/main.splash`):
- 移除 `instructions`(已并入 `task` 文本)
- 移除 `output` 包装
- 移除 `temperature` / `max_tokens`(由 host 按 `class` 选模型)
- `class: "fast"` / `class: "strong"` 上提到顶层

并修正 `ai_budget()` 字段读取(`ledger.rs:75-83` `Budget::to_json` 实际键为 `calls_today/calls_per_day/tokens_today/tokens_per_day/tokens_left/per_minute/resets_at`,旧代码读的是不存在的 `remaining_today/tokens_remaining/calls_remaining`,总是显示 `=—`)。

#### 3. 真实 host 集成测试(11 个新测试,全 PASSED)

新增文件:`apps/ai-providers/host-service/tests/octostudio_calls.rs`(单文件,**真实走 `octosense_appstore::services::dispatch`** + `complete::register_with`,仅替换 `Transport` 为假 provider;不 mock `Request::from_args` / `accept()` / schema 校验)。

```
$ cd /Volumes/PSSD/CodeProjects/OctoSense && \
  cargo test --test octostudio_calls -p octosense-llm-service -- --test-threads=1

running 11 tests
test composition_args_pass ... ok
test m4_budget_args_pass ... ok
test m4_keywords_args_pass ... ok
test m4_score_title_args_pass ... ok
test m4_style_variants_args_pass ... ok
test m4_summary_args_pass ... ok
test m4_title_args_pass ... ok
test m4_translate_args_pass ... ok
test old_m4_shape_with_instructions_output_wrapper_is_rejected ... ok
test scenario_image_args_pass ... ok
test scenario_video_args_pass_with_preset ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

11 个测试覆盖:
- **6 个 M4 调用**:标题 / 关键词 / 摘要 / 风格 / 翻译 / 打分 — 全部从 `bundle/main.splash` 实际 args 复制,**真实 dispatch**,验证 host 接受 + schema 校验通过 + 解析出 `r.data["output"]` 中的字段。
- **2 个场景调用**:`image` 与 `video`(含 preset 注入)。
- **1 个 composition 调用**:视频合成建议(转场/配乐/节奏/调色/字幕/输出)。
- **1 个 budget 调用**:验证 6 个实际字段名 + 旧字段名不存在(`remaining_today`/`calls_remaining` 为 `None`)。
- **1 个回归保护**:旧 shape(`instructions` + `output` 包装 + `temperature` + `max_tokens`)被 host 拒收,返回 `bad_request: ... does not take ...`。

#### 4. 全栈 `cargo test` 实测结果

```
$ cd /Volumes/PSSD/CodeProjects/OctoSense
$ cargo test -p octosense-llm-service --tests -- --test-threads=1
test result: ok. 30 passed; 0 failed   # image.rs
test result: ok. 19 passed; 0 failed   # service.rs
test result: ok.  8 passed; 0 failed   # unit tests
test result: ok. 11 passed; 0 failed   # octostudio_calls.rs (本轮新增)
test result: ok. 14 passed; 0 failed   # image (1 ignored)
test result: ok. 20 passed; 0 failed   # service (sheet tests)
合计: 102 passed; 0 failed

$ cargo test -p octosense-ai-host --lib
test result: ok. 37 passed; 0 failed

$ cargo test -p octosense-shell --lib
test result: ok. 787 passed; 0 failed
```

**全栈 926 个测试全 PASSED**,含本轮新增的 11 个 octostudio_calls。零回归。

#### 5. 可生产级内容产出判定

| 维度 | 状态 | 证据 |
|---|---|---|
| **场景覆盖** | ✅ 10 场景齐全(9 结构化 + 1 原文二创),覆盖图文/分镜/PPT/拆解/标题/小红书/口播/导图/金句 | `SCENARIOS` 注册表 main.splash:145-156 |
| **Schema 严格性** | ✅ 每个场景独立 JSON Schema,`additionalProperties: false` + `required` + `min/maxItems` + `maxLength` | `scenario_schema()` main.splash:1094-1234,每条规则均在 host 的 schema.rs 子集内 |
| **变量注入** | ✅ 10 主题 × 4 文章类场景 = 40 个 tone 注入点;5 预设 × video = 5 个风格注入点 | `theme_prompt()` `preset_line()` main.splash:359-377 |
| **降级链路** | ✅ card-host / 无 model 时 → `load_demo_plan()` 填入演示内容,跳转编辑器可手动保存 | `ask_scenario` line 1831-1837,`load_demo_plan` line 1431-1494 |
| **可编辑性** | ✅ 通用计划编辑器支持增/删/改/排;条目按 `_k` 稳定追踪 | `item_*` 系列 main.splash:1934-2051 |
| **导出完整性** | ✅ 7 种格式 × 10 场景 = 至少 28 个导出组合,文本与原文 / 配图 prompt / SRT / Marp / markmap 都可复制 | `kind_formats` line 273-287 + `render_plan_as_text` line 2329-2352 |
| **args 形状合法** | ✅ 11 个 dispatch 集成测试 PASSED | octostudio_calls.rs(本轮新增) |
| **实际生成质量** | ⚠️ 受宿主 model provider 影响;应用层任务模板 + schema + 降级链路完整,provider 选型由用户在系统 AI providers 设置 | (本机无完整 Shell runtime 测试,但 args 形状 + schema 校验已真实验证) |

#### 6. 结论

- **AI 依赖核心 = 提示词 + 场景模板 + 变量机制,经本机源码级核实全部完备**。
- **场景闭环**:`SCENARIOS` 注册表 → `scenario_task` → `scenario_schema` → `scenario_input` → `absorb_output` → `render_plan_as_text` 五件套一一对齐,无断点。
- **修了一个真实 bug**:v0.4.0-v0.4.2 的 M4 6 个 `model.complete` 调用 args 形状非法(v0.4.3 修复后通过 host 集成测试)。
- **修了一个真实 bug**:v0.4.0-v0.4.2 的 `ai_budget` 读取字段名错误,显示永远是 `=—`(v0.4.3 修正为 `calls_today/tokens_today/tokens_left/per_minute` 等实际键)。
- **真实可生产级**:仅受模型 provider 选型影响(provider 由用户在 OctoSense Shell 系统设置里配置),应用层不引入密钥、不限模型,严格走 schema + 降级链路,在 card-host / 无 model / model 失败三类情况下都给出可读错误 + 手动兜底。


### 框架级缺口(已上报)

下列能力在 OctoSense `main` (2026-10-01) 与 App Hub `main` 尚未发布,文档也未列出。本轮已在对应仓库提 issue,详见「已提 issue」列:

- **文生图(`model.image` / `octos.image`)** — 商店应用唯一的"图像"产出途径仍是 `images` capability 加载外部 URL(配图降级为文本),详见 ROADMAP M7 等平台。OctoSense-App-Hub issue 提请:`Add model.image and octos.image host services for image generation`.
- **文生视频(`model.video` / `octos.video`)** — 当前零路径;视频 widget 同样未发布。OctoSense-App-Hub issue 提请:`Add model.video / octos.video host services for text-to-video`.
- **TTS(`model.audio`)** — 文字转语音无服务。OctoSense-App-Hub issue 提请:`Add model.audio for TTS host service`.
- **嵌入(`model.embeddings`)** — 无服务。OctoSense-App-Hub issue 提请:`Add model.embeddings for vector embedding host service`.
- **`sys.digest`** — 准入检查需要 `sys` 能力。当前 manifest 尚未声明(SPEC 已加入 ROADMAP M7 等平台定项,OctoSense PR 待合)。

### 关于 v0.4.0 的降级声明

- card-host **不连接** octos/model/glance — 这是设计如此,不是 bug。
- 商店应用进入真实 OctoSense Shell 后,用户首次使用会看到「Waiting for the person to allow this app's agent」对话框;一旦允许,`octos.*` 全部可用。
- 模型提供方由用户在 AI providers 系统应用里配置;应用包内**绝不可**放任何 token 或私钥(已经过 `hub check` 验证)。
- 当前 v0.4.0 的所有 AI 功能在「未连接」状态下都给出可读错误并保留手动编辑保存路径。

未来规划(见 [`ROADMAP.md`](ROADMAP.md)):个人风格工坊、主题网络抓取、拟人化文章生成、glance 卡片分发。

## 5 分钟走一遍

> 完整流程与画面见上方 [60s 视频](#) — 这里只列文字版。

1. **逛创作广场** — 10 张场景卡一眼排开,下面是本地作品库
2. **选主题样式** — 点「图文文章」,挑一款「文章主题样式」(如 🌙 情感散文),写下一句话意图
3. **图文 plan + AI 配图** — 每段一个卡片:标题/正文/生图 prompt,卡片下方是可选增强的真实 AI 配图(离线自动忽略)
4. **编辑器随手改** — 点「编辑」就地展开面板(留空 = 不修改),上移/下移/删除/新增随心意
5. **一句话生成 PPT** — 「PPT 演示」场景出 5–10 页大纲(要点/讲稿/配图 prompt)
6. **视频:分镜 → 合成建议 → 制作包** — 生成 4–6 镜后点「AI 合成建议」得到转场/配乐/调色;导出页切「SRT字幕」拿时间轴字幕,切「制作包」拿到素材清单 + SRT + 合成建议 + ffmpeg 参考命令,照做即成片
7. **Marp 一键成演示** — PPT 场景导出 Marp 文本,粘到 marp.app 即放映/导出 PDF
8. **拆解一个爆款** — 粘文字稿进「拆解视频」,得到结构/钩子/镜头语言/金句与可复用骨架,「存为分镜模板」直接变成你的下一支片

> 完整流程与画面见上方 60s 视频;`bundle/screenshots/` 保留 8 张实拍用于 App Hub listing。

## 设计理念

> **言出法随 · 意图即应用** — 一句话即产物,意图即路径。

五条核心信念,贯穿所有屏幕:

1. **言出法随** — 用户输入与产物之间不留缝隙;一切交互以"一句话达成"为标尺。
2. **意图即应用** — 应用形态由意图驱动,不提供"配置应用"的中间层;AI 是意图的创造性填充。
3. **本地优先** — 作品存设备沙箱,跨设备同步靠 OctoSense 而非云端账号;**无密钥 / 无 token / 无账号注册**。
4. **平台原生 AI** — 在 Rinx / OctoSense 设备上调用宿主 AI(`octos.*` + `model`),用户密钥由 AI providers 系统应用托管,应用本身看不到模型 id 或 API key。
5. **透明降级 + 多格式出口** — AI 不可用时 UI 仍可手动创作并保存,失败原因原样显示;创作链的最后一站对接到真实发布平台(公众号 / Markdown / Notion)。

## 平台能力

OctoStudio 用到的 OctoSense 平台能力(声明在 `bundle/manifest.json`):

| 能力 | 角色 | 落在哪些屏幕 |
|---|---|---|
| `octos.session.open / octos.turn.start` | 设备 AI peer,原文二创 ×8 风格 | 工坊屏 `AI 创作` |
| `model.complete` | 一次性、按 schema 校验的模型调用;全部 9 个生成场景 + AI 合成建议的主路径 | 录入屏 `生成`、计划屏 `AI 合成建议` |
| `images` | 显示参考图;**AI 配图增强**(免鉴权公网生图服务,离线自动降级为纯 prompt 文本) | 工坊参考图、计划屏卡片配图 |
| `web` | 打开公开 https 网页(预留,本 v0.3 未主动调用) | — |
| `storage` | 作品库落盘(含 v0.2 数据自动迁移) | 所有屏 |

应用本身不持有任何密钥或 token,所有 AI 请求由宿主服务代理。

## 不做什么

**平台红线(永不做)**:

- 不申请 `llm / news / mail / profile`(仅系统应用可用,商店申请必拒)
- 不收集密码 / PIN / 一次性码
- `network.hosts` 保持空数组,所有 https 资源走 `images`(公开域)或用户回填的 URL;`net` capability 本 v0.2 不申请
- 不写 `os.*` id(这是商店应用,不是系统应用)
- 不在 bundle 中放 token / API key / 私钥

**v0.3 不做(规划中)**:

- 应用内端到端视频渲染/播放 — 平台无 video widget;「导出视频」的落地形态是**制作包**:逐镜 prompt + SRT 字幕 + ffmpeg 参考命令,外部工具照做即成片
- 需要密钥的商用生图/生视频 API — bundle 禁止密钥;AI 配图走 `images` capability 允许的免鉴权公网服务,可选可关,离线不影响任何功能
- 一键发公众号 — 凭据不入包,保持"复制粘贴到后台"的克制设计

**降级路径(明示)**:

- `card-host` / 旧 App Hub pin 不提供 `octos.*` / `model`:UI 全程可手动创作并保存,所有 AI 失败显示 `no service answers "<svc>" on this device` 原文
- `model` 不可用时,全部 9 个场景填入预置 demo 内容 + 状态栏写 `model 不可用(原因),已填入演示内容`
- 微信 OAuth / secret:不接,人到后台粘贴

## 接下来

完整迭代路线见 [`ROADMAP.md`](ROADMAP.md)。当前状态:

| 里程碑 | 版本 | 状态 |
|---|---|---|
| M0 / M1 意图创作 + 多格式导出 | v0.2.0 | ✅ 已落地 |
| M2 创作者工作流补齐 | v0.2.x | ✅ 已落地 |
| M3 场景广场(9 生成场景 + 通用编辑器 + 主题样式 + 制作包/Marp/markmap) | v0.3.0 | ✅ 已落地 |
| M3' 内容管理(搜索 / 标签筛选 / AI 历史) | v0.3.5 | ✅ 已落地 |
| M3" 排序 + 多选 + 撤销 + 一键复制 | v0.4.0 | ✅ 已落地(本轮) |
| M4 `model.complete` 高阶用法 | v0.4.0 | ✅ 已落地(本轮):AI 助手 7 项(起标题/关键词/摘要/风格迁移/中英对照/标题打分/模型预算) |
| M5 glance 卡片 | v0.4.0 | ✅ 能力声明已加(`glance` capability + `glance.publish`/`glance.withdraw` 助手);UI 渲染待 Splash VM 修复 |
| M6 拟人化文章生成 | v0.5.0 | 规划中 |
| M7 等平台(视频 widget / 触发器 / toolbox-peers / `sys.digest`) | — | 等 OctoSense |

---

## 寄语

> **言出法随,意图即应用。**
>
> 愿每一个写作者,开口即成文。
> 愿每一个意图,落地即产物。

# 给开发者

下面这一节是给协作者看的 — 怎么把工具链跑起来、怎么改 `main.splash`、怎么发版。初赛评审可跳过。

## 前置条件

按 [`OctoScript-App-Design-Flow/README.zh-CN.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/README.zh-CN.md) 的快速上手:

| 工具 | 版本 | 备注 |
| --- | --- | --- |
| macOS(Apple silicon 已验证) | 14+ | Windows / Linux 未验证 |
| Rust(stable via rustup) | `~/.cargo/bin` 加入 PATH | 用来构建 `hub` 与 `card-host` |
| Python | 3.9+ | macOS 自带的 `/usr/bin/python3` 即可 |
| 图形会话 | — | card-host 会开一个 412×803 点的窗口(retina 屏上像素 824×1606) |

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

## 一次性构建工具链

进入工作区,从 `OctoSense-App-Hub` 编译两个二进制(总产物约 1 GB,首次约 1 分 30 秒):

```sh
cd <workspace>
git clone https://github.com/OctoSense-org/OctoScript-App-Design-Flow.git
git clone https://github.com/OctoSense-org/OctoSense-App-Hub.git
cd OctoScript-App-Design-Flow && python3 tools/setup-native.py

# 这一步会拉 makepad / octoscript / octoscript-makepad 三个仓库并锁定到
# native-runtime.lock.json 指定的 commit。可重复跑、幂等;三个仓库如果已
# 经在工作区里、版本对得上,这一步只是 no-op,直接 cargo build 也行。
(cd ../OctoSense-App-Hub && cargo build --release -p octosense-card-host -p octosense-app-hub)

tools/octo doctor    # 应该全部 [ok]
```

**跳过 clone** — 如果工作区里已经有这五个仓库(本机开发日常就是这种状态),直接 `cargo build --release` 即可,不需要 `setup-native.py`。

**产物位置与 `octo` 怎么找到它** — 默认 cargo 把 release 产物放在 `OctoSense-App-Hub/target/release/`,但很多人(本机开发)会设 `CARGO_TARGET_DIR` 把所有 rust 项目的产物集中到一块 NVMe 上:

```sh
export CARGO_TARGET_DIR=/Volumes/PSSD/dev/rust-target
# 这样 cargo build 产物落到 /Volumes/PSSD/dev/rust-target/release/{hub,card-host}
```

`octo` 找二进制的顺序是 `OCTO_HUB` / `OCTO_CARD_HOST` 环境变量 → `$OCTOSENSE_APP_HUB/target/release` → **`$CARGO_TARGET_DIR/release`** → 固定 fallback 路径 → `$PATH`。只要二进制落在其中任意一处,无需任何配置;只有当你想把多个 release 目录切来切去,才用 `OCTO_HUB` / `OCTO_CARD_HOST` 显式指定:

```sh
export OCTO_HUB=/path/to/hub
export OCTO_CARD_HOST=/path/to/card-host
```

**怎么知道当前到底用哪个** — `tools/octo doctor` 会把每条 `[ok]` 后面跟来源写出来,比如 `(from /Volumes/PSSD/dev/rust-target/release)` 或 `(from $OCTO_HUB)`。

> **重新编译的触发条件** — `OctoSense-App-Hub` / `octoscript-makepad` / `octoscript` 任意一个 `git pull` 后,cargo 的增量编译会自动检测并只重编 dirty crate;无需手动 `cargo clean`。跨大版本(比如 Splash VM 字节码格式变)才需要 `cargo clean && cargo build --release`。

## 仓库结构

```text
octostudio/
├── README.md                ← 你正在读
├── BRIEF.md                 设计意图
├── REVIEW-ANSWERS.md        答 hub scan 7 个审核问题
├── ROADMAP.md               迭代路线
├── build/                   hub scan packet(不进 git)
├── bundle/                  ★ THE SUBMISSION
│   ├── manifest.json        id、name、version、capabilities、hosts、integrity
│   ├── listing.json         商店展示的元数据
│   ├── main.splash          程序本体(736 行,Splash 脚本)
│   ├── assets/icon.svg      列表上的图标
│   └── screenshots/         8 张真实截图(01 / 02 / 03 / 04 / 06 / 07 / 08 / 09)
├── AGENTS.md  CLAUDE.md  GEMINI.md   Agent 规则(从模板复制,不提交)
├── .gitignore               忽略 build/ / target/ / .local-state/ / *.key
└── .local-state/            card-host 的 jail + log(本地调试用,不提交)
```

**`main.splash` 本身就是程序**。它不是配置文件,没有 .py / .rs / .ts 配套。整个应用是一个被 Splash VM 解释执行的脚本应用,顶层 `let` 是状态、`fn` 是函数,然后一个根 View。

## 看到页面 — 三种模式

### 0. 先 `octo doctor`,再跑

工具链装完之后,先确认所有依赖找得到:

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo doctor
```

期望输出(全部 `[ok]`):

```text
octo doctor  (repo /Volumes/PSSD/CodeProjects/OctoScript-App-Design-Flow)
  [ok]   python 3.14.7
  [ok]   App Hub checkout /Volumes/PSSD/CodeProjects/OctoSense-App-Hub
  [ok]   hub: /Volumes/PSSD/dev/rust-target/release/hub  (from /Volumes/PSSD/dev/rust-target/release)
  [ok]   card-host: /Volumes/PSSD/dev/rust-target/release/card-host  (from /Volumes/PSSD/dev/rust-target/release)
  [ok]   cargo: /Users/lijing/.cargo/bin/cargo
  [ok]   template templates/script-app
```

`octo` 找二进制的顺序是 `OCTO_HUB` / `OCTO_CARD_HOST` 环境变量 → `$OCTOSENSE_APP_HUB/target/release` → `$CARGO_TARGET_DIR/release` → 固定 fallback 路径 → `$PATH`。常见找不到的情况:

- **编译产物不在默认路径** — 如果你设了 `CARGO_TARGET_DIR`(本仓库开发机就是 `CARGO_TARGET_DIR=/Volumes/PSSD/dev/rust-target`,产物落到 `…/dev/rust-target/release/`),`octo` 会自动从 `$CARGO_TARGET_DIR/release` 找,无需配置。
- **环境变量显式指向其它路径** — `export OCTO_HUB=/path/to/hub OCTO_CARD_HOST=/path/to/card-host` 即可覆盖搜索路径。
- **二进制真的没编译** — 回到上一节 `cargo build --release -p octosense-card-host -p octosense-app-hub`,首次约 1m30s,产物 ~1 GB。

### 1. 跑起来的最小命令

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo run /Volumes/PSSD/CodeProjects/octostudio/bundle \
    --port 8141 --detach
```

> **电脑端 vs 手机端** — card-host 默认窗口是 **412×892**(手机竖屏尺寸,`host.rs:22` 写死的 `window.inner_size: vec2(412, 892)`)。需要电脑端窗口用 `--size WxH`:
>
> ```sh
> # 手机(默认,可不写)
> tools/octo run bundle --port 8141 --detach
>
> # 平板 1024×768
> tools/octo run bundle --port 8141 --detach --size 1024x768
>
> # 桌面 1280×800
> tools/octo run bundle --port 8141 --detach --size 1280x800
> ```
>
> splash 内部布局按"单列 + Fill 宽度"组织:手机窗口正常铺满,桌面窗口内容靠左、右半屏留白。Splash VM 没有暴露 `host.window_size()` 给脚本、也没有原生 breakpoint,所以 v0.2 没有真正的响应式断点;`--size` 是当前把窗口尺寸交给开发者的官方口子。
>
> 已知不可行的几种"真响应式"尝试(避免重复踩坑):`flow: RightWrap` + 固定宽度子元素在 card-host 下不按预期换行;`AdaptiveView{Mobile,Desktop}` 在 `on_render` 复杂回调下会触发 card-host `gen_index.rs` panic。等 OctoScript 加 `host.window_size()` / `parent.width()` API 后再做 v0.3 真响应式。
```

`--detach` 在 macOS 上会让 card-host 开一个窗口(fork 到后台,shell 立即返回),看到 ready 行就算起来了:

```text
octo: /Volumes/PSSD/dev/rust-target/release/card-host --bundle ... --app-data ... --allow-unsigned --stamp
ready: first frame drawn
pid 32429  log /Volumes/PSSD/CodeProjects/octostudio/.local-state/card-host.log
drive it: curl -s 127.0.0.1:8141/snap | tools/octo shot 8141 <out.png> | curl -s 127.0.0.1:8141/quit
```

`octo run` 实际隐式加了两个 flag:

- `--allow-unsigned` — 跳过 publisher 签名检查,本地 dev 必备;签了名再发布时会自动跳过
- `--stamp` — 用最新 `main.splash` 重算 `bundle_blake3` 并写回 `manifest.json`;不想覆盖就显式 `--no-stamp`

### 2. Smoke test — 确认真的活了

`--detach` 只代表 fork 成功,首屏可能还在加载。**用下面三条命令做 3 秒健康检查**:

```sh
# 1) 进程在吗?
pgrep -fl card-host | head -1
# 期望: ... /Volumes/PSSD/dev/rust-target/release/card-host --bundle ...

# 2) 远程桥活着吗? (GET / 列路由)
curl -s 127.0.0.1:8141/ | head -1
# 期望: makepad-remote  app=card-host pid=32429  windows=1 ...

# 3) 拍一张真截图看渲染对不对
tools/octo shot 8141 /tmp/octo_smoke.png
open /tmp/octo_smoke.png
# 期望: 顶部有 "OctoStudio" 标题 + 5 个 tab(广场/录入/工坊/计划/导出),广场有 2×5 场景卡 + 作品卡(本机已 seed 过)
#       或者 "还没有作品,点新建开始一段创作。"(全新 jail,首次启动)
```

`works.json` 状态决定首页长什么样:

| 状态 | 文件 | 看到 |
| --- | --- | --- |
| 全新 jail | `.local-state/octostudio/works.json` 不存在 | "还没有作品,点新建开始一段创作。" + 6 tab |
| 已 seed(本仓库当前) | `.local-state/octostudio/works.json` 存在 | "创作广场" 标题 + 2×5 场景卡 + N 张作品卡 |
| 想清空重来 | `rm .local-state/octostudio/works.json` 再重启 card-host | 回到全新状态 |

### 3. 停下来

```sh
# 优雅退出(推荐 — 会等 in-flight 渲染结束)
curl -s 127.0.0.1:8141/quit

# 兜底(上一个 quit 没响应 / 端口错)
pkill -f card-host
```

只 kill 进程没问题 — `.local-state/octostudio/works.json` 是原子写,半路中断不会损坏。**重启后所有作品、字数控件、style 字段都还在**。

---

### 模式 1:Agent / 脚本驱动(无头,推荐)

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo run /Users/lijing/CodeProjects/octostudio/bundle \
    --port 8142 --hidden --detach

# 输出示例:
#   card-host: octostudio 0.2.0 admitted — capabilities {…}
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

去掉 `--hidden`,card-host 会弹一个 412×803 点的窗口(retina 上 824×1606 像素),你直接点。

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
tools/octo run /Volumes/PSSD/CodeProjects/octostudio/bundle --port 8142 --hidden --detach

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
# 点 home 页的 "图文" 按钮(顶部右侧,坐标 x=275 y=181 是 CSS 点)
curl -s "127.0.0.1:8142/click?x=275&y=181&wait=1"

# 输入文本(需先 focus 输入框 — 上面那个 click 已经 focus 了)
curl -s --get "http://127.0.0.1:8142/t" --data-urlencode "t=夏日海边慢生活"
```

完整截图捕获:

```sh
tools/octo shot 8142 /Volumes/PSSD/CodeProjects/octostudio/bundle/screenshots/01-plaza.png
```

> 截图是在 **App 自身渲染管线**里出的图(应用自己把 PNG 写到磁盘),即使在 `--hidden` 下也能拿到完整画面。

### 看日志(出错时第一件事)

```sh
tail -30 /Volumes/PSSD/CodeProjects/octostudio/.local-state/card-host.log
# 找 [E] splash: line:col - … 错误行
# [SPLASH] eval: NN bytes 后面是编译错误
```

如果你看到 `method X not found on object` — 那是 Splash 脚本里调用了不存在的 API。检查名字、对照 [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md)。

### 调试脚本:`sys.time_ms()` 之类的坑

Splash isolate **剥离**了 `mod.fs` / `mod.run` / `mod.res` / `mod.cx.quit`,这几个全局名在脚本里都是 `nil`(见 `widgets/src/widget_async.rs:562`)。如果你看到 `variable X not found`,多半是这几个之一。

时间戳用 `std::time::SystemTime` 也拿不到 — Splash 没有暴露这个 API。OctoStudio 里用一个 `seq` 单调递增计数器代替。随机数同理,自己做 seed。

## 跑 `octo check`(准入检查)

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo check /Volumes/PSSD/CodeProjects/octostudio/bundle
```

期望输出(v0.2.0):

```text
octostudio 0.2.0 — PASSED
  [warning] publisher-signature: unsigned: accountability rests on the hub alone
  grants: capabilities {"images", "model", "octos.session.open", "octos.turn.start", "storage", "web"}, hosts {}, storage 16777216 bytes, agent none
```

`check` 先 `hub stamp` 写入 `bundle_blake3` 到 `manifest.json`,再 `hub check --allow-unsigned`。如果拒绝,看具体哪个权限或 listing 字段被拒。

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
- 闭包形式的动态生成,**用 `set_visible` + `set_text` 触发重绘**(`render()` 在某些 View 上不稳定)
- 切屏:`ui.a.set_visible(false); ui.b.set_visible(true)`(`Overlay` 父容器里)
- 模态:`show_dialog(modalId)`

API 全部见 [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md);Gotchas 部分节省 80% 的调试时间。

## 发布到 App Hub

`tools/octo package-help` 会输出完整清单。简要步骤(全是人手):

```sh
# 1. 修掉 listing.json 里的模板占位
#    publisher.name: "AmosLi (TODO 替换为发布者名称)" → 你的名字
#    publisher.support / privacy_policy_url: "https://example.com/..." → 真实 URL

# 2. 生成签名密钥(仓库外保存,只跑一次)
hub keygen /secure/path/publisher.key

# 3. 签名
hub sign-manifest /Volumes/PSSD/CodeProjects/octostudio/bundle \
  --key /secure/path/publisher.key --key-id aios.pub

# 4. 带密钥重检
hub check /Volumes/PSSD/CodeProjects/octostudio/bundle \
  --publisher-key aios.pub=$(hub pubkey /secure/path/publisher.key)

# 5. 在 GitHub 上打 tag
git tag octostudio-0.2.0 && git push origin octostudio-0.2.0

# 6. 在 OctoSense-App-Hub 开 issue
#    标题:Submit octostudio 0.X.Y
#    附:tag、commit、bundle 路径、publisher 公钥、check 输出、REVIEW-ANSWERS.md
```

### v0.3.3 提交流程(publisher `aios.pub`,2026-10-05 实测通过)

完整记录在 `build/ISSUE-SUBMIT-0.3.3.md`,顺序是:**bundle 修改 → stamp → sign → 重发本地 mirror → commit & tag → push → 更新 issue**。任何一步之后改了 bundle 都要回到第一步重做。

```sh
APP=/Volumes/PSSD/CodeProjects/octostudio
HUB=/Volumes/PSSD/dev/rust-target/release/hub
KEYS=$APP/build/keys
M=$APP/build/mirror
KEY=~/.octosense/aios.pub-publisher.key            # publisher 私钥
PUBKEY=$($HUB pubkey $KEY)                          # 当前值 c02572b30ef0c38a…

# 1. stamp(把 bundle_blake3 写入 manifest)
$HUB stamp bundle

# 2. sign(用 aios.pub 密钥签 manifest)
$HUB sign-manifest bundle --key $KEY --key-id aios.pub

# 3. 签名自检(通过表示 signature value 与 manifest 字节一致)
$HUB check bundle --publisher-key "aios.pub=$PUBKEY"
# 期望: octostudio 0.3.3 — PASSED

# 4. 重发本地 mirror(写入 build/mirror/catalog.json + anchor.hex)
ANCHOR=$($HUB pubkey $KEYS/anchor.key)
CERT=$($HUB certify --anchor $KEYS/anchor.key --working $KEYS/working.key)
$HUB publish bundle --catalog $M/catalog.json \
  --key $KEYS/working.key --anchor-cert "$CERT" \
  --publisher aios.pub \
  --publisher-key "aios.pub=$PUBKEY" \
  --repo https://github.com/aios-pub/OctoStudio.git \
  --commit "$(git -C $APP rev-parse HEAD)" --out $M
$HUB verify $M/catalog.json --anchor "$ANCHOR"
echo $ANCHOR > $APP/build/anchor.hex

# 5. commit 本地变更(分支名带 publisher,清晰)
git checkout -b v0.3.3-aios.pub
git add bundle/ README.md INTEGRATION.md REVIEW-ANSWERS.md ROADMAP.md
git commit -m "v0.3.3-aios.pub: ..."
git tag octostudio-0.3.3                              # lightweight tag

# 6. push 到 aios-pub/OctoStudio
# 6a. 正常网络:
git push origin v0.3.3-aios.pub --follow-tags
# 6b. 443 超时(macOS Clash 网关常见):走 gh api,见下面"GitHub 推送变通"
```

> ⚠️ **签名严格不可逆** —— manifest 改了就要重签;本地 catalog 用了就不要再重签旧 digest。

### GitHub 推送变通(`git push` 443 超时时)

`gh api` 直连 GitHub 是不走 git 协议的,不受 443 timeout 影响。完整脚本见 `~/.zcode/cli/memories/projects/octostudio-69a6319f7f84b652/memory/github-push-via-gh-api.md`,核心步骤:

```python
# 1) POST /git/blobs(逐文件 base64)
# 2) POST /git/trees(递归建子树)
# 3) POST /git/commits(parent = 远端 main 当前 SHA,tree = 新 root)
# 4) POST /git/refs(branch + lightweight tag)
```

GitHub 侧重建的 commit SHA 与本地不同(时间戳格式差异),但 **tree SHA 完全一致**(因为 blobs + tree 是逐字段构造的)。验证:
```
gh api repos/aios-pub/OctoStudio/branches/v0.3.3-aios.pub
gh api repos/aios-pub/OctoStudio/tags
```

### 当前 v0.3.3 远端状态(已推送)

| 项 | 值 |
|---|---|
| Issue | [#73 — Submit octostudio 0.3.3 (publisher: aios.pub)](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/73) |
| Tag | [`octostudio-0.3.3`](https://github.com/aios-pub/OctoStudio/tree/octostudio-0.3.3) → `0e63cf1fb25c` |
| Branch | [`v0.3.3-aios.pub`](https://github.com/aios-pub/OctoStudio/tree/v0.3.3-aios.pub) |
| Local commit | `f76afe0`(本地) — 同 tree (`ce4a1fab267d`)、作者、时间戳 |
| Bundle digest | `0d81c8a5409f8d9fe58d66fc27574232ab4af914093d703d7342b9279bea14e2` |
| Publisher pubkey | `c02572b30ef0c38a56de97b909fe6fbb419212ea3d8723d0b0342b7737394c02` |
| hub check | `octostudio 0.3.3 — PASSED`(`hub check bundle --publisher-key aios.pub=…`) |
| Local mirror | `catalog sequence 1 verified, 1 entries`(`build/mirror/anchor.hex = 96f4f77f…`) |

Reviewer 在 hub 完成注册后,可走 route **human-review**(与 v0.1.0 / v0.3.0 一致)。`REVIEW-ANSWERS.md` 已随 commit 推送。

签名之后任何修改都要重新 stamp + 签名。

## 故障排除

| 现象 | 原因 | 修复 |
| --- | --- | --- |
| `octo doctor` 报 `hub not found` / `card-host not found` | 二进制没编译,或 `CARGO_TARGET_DIR` 不在搜索路径 | 跑一次 `cargo build --release -p octosense-card-host -p octosense-app-hub`;如果有自定义 `CARGO_TARGET_DIR`,看 doctor 输出确认 `from /Volumes/PSSD/dev/rust-target/release`;想硬指就 `export OCTO_HUB=… OCTO_CARD_HOST=…` |
| `octo doctor` 报 `App Hub checkout … not found` | `OctoSense-App-Hub` 不在工作区同级目录 | `git clone https://github.com/OctoSense-org/OctoSense-App-Hub.git` 到 `<workspace>/OctoSense-App-Hub`,或 `export OCTOSENSE_APP_HUB=/path/to/OctoSense-App-Hub` |
| `port 8142 already in use` | 上一个 card-host 没退 | `pkill -f card-host` 或 `curl -s 127.0.0.1:8142/quit` |
| `--detach` 后窗口没出来 | macOS 上 `--detach` 默认是 offscreen / focused | 改用 `tools/octo run bundle --port 8141`(去掉 `--hidden`),窗口就出现;或者保留 `--detach` 但用 `tools/octo shot 8141 out.png` 拍远程截图,看不到窗口也能验证 |
| `card-host` 起来了但 `curl /snap` 连不上 | 端口被防火墙拦 / 进程其实没起来 | `lsof -nP -iTCP:8141 -sTCP:LISTEN`;`tail -20 .local-state/card-host.log` 找 `[E]` 行 |
| `[E] splash: line:col - method X not found` | 调了不存在的 API | 查 `docs/SCRIPT-API.md` |
| `[E] splash: line:col - variable X not found` | 用了 `fs / run / res / cx.quit` 之一 | 这些在 isolate 里被剥成 nil,改用自己的实现 |
| 首屏出来后一直是 loading | `start_timeout(0.05, \|\| boot())` 没跑,或 boot 报错 | 看 `card-host.log` 的 `[SPLASH] eval:` 后面 |
| `AI 创作` 按钮点了之后状态不变 | `host.request("octos.turn.start", …)` 返回 `{ok: false, error: …}` | card-host 不提供宿主服务 — 这是 README §"降级路径"描述的降级,UI 应给出 `no service answers` |
| `生成大纲` 卡在 "生成大纲中…" | model 不可用且 demo 分支没触发 | 看 status_text 是否被覆盖;检查 `ask_intent_plan()` 回调分支 |
| 截图里输入框是空的 | 输入后没等渲染就 shot 了 | 加 `--settle 2`(默认 2 秒) |
| chip 切换后子控件不变 | `visible: (expr)` 只求值一次 | 用 `set_visible()` 程序控制 |
| 改了 main.splash 没生效 | card-host 还在跑旧版本 | `pkill -f card-host` 再重启 |
| `hub check` 报 `refused listing: …` | listing 引用了不存在的文件 | 真实截图放进 `bundle/screenshots/` |
| `hub check` 报 `bundle integrity changed since stamp` | `main.splash` 改了但没 `--stamp` | `tools/octo run` 会自动 stamp;手动 `hub stamp` 后再 `hub check` |

## 相关链接

- [App Hub issue #73 — Submit octostudio 0.3.3 (publisher: aios.pub)](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/73) — 当前提交工单(已含 tag、commit、bundle 信息、publisher 公钥)
- [`OctoScript-App-Design-Flow`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow) — 工具链与文档
- [`OctoSense-App-Hub`](https://github.com/OctoSense-org/OctoSense-App-Hub) — `hub` / `card-host` / 商店
- [`docs/SCRIPT-API.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md) — Splash 语言 + 全部 API
- [`docs/CAPABILITIES.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/CAPABILITIES.md) — 每种权限解锁什么、用户看到什么
- [`docs/AI-SERVICES.zh-CN.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/AI-SERVICES.zh-CN.md) — `octos.*` / `model.*` 当前能做什么、规划
- [`docs/PUBLISHING.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/PUBLISHING.md) — 发布到 App Hub
- [`BRIEF.md`](BRIEF.md) — 设计意图
- [`REVIEW-ANSWERS.md`](REVIEW-ANSWERS.md) — 答 hub scan 7 个审核问题
- [`ROADMAP.md`](ROADMAP.md) — 迭代路线

## 许可证

Apache-2.0。
