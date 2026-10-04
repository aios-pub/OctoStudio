# OctoStudio · Roadmap

> 本文件不是商店提交物;与 `BRIEF.md` / `REVIEW-ANSWERS.md` 同级,只用于项目自身的
> 迭代规划。所有"可用 / 暂缓"判断的依据是
> [`docs/AI-SERVICES.zh-CN.md` 状态一览](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/AI-SERVICES.zh-CN.md)
> (App Hub `a72989f` + OctoSense `7082ff5`,核对日期 2026-09-30)。

## 原则

1. **优先实现当下已经支持的** —— `model.complete`、`octos.*`、`glance.*`、`images / web / storage / camera`、
   工具注册到 peer 等都已合并到 App Hub 与 Shell 的 `main`,OctoStudio 现在就能用。
2. **暂缓未发布的** —— 触发器 / 后台运行 / 应用记忆 / 工具箱 → 应用 Agent / `sys.digest`
   仍在 PR 或 ADR 中;`octos.*` 与 `model.*` 均无 image / video 原生输出。文档里写"即将推出"
   的,本路线图只列为"等平台",不预先动工。
3. **每一步可回滚、可截图** —— 每个里程碑结束时都要跑 `tools/octo check bundle/` 并
   重抓 `bundle/screenshots/` 里的真实截图;改动不通过就回滚。
4. **不破坏 v0.1.0 数据契约** —— `works.json` 字段只追加、不删改;新字段缺席时旧作品
   应正常打开。
5. **不在 bundle 中引入密钥 / token / 账号**;`network.hosts` 仍为空,所有 https 资源
   只走 `images` 公开域或用户回填的 `image_url / video_url`。

---

## 一句话

把 OctoStudio 从"原文 → 改写 / 翻译 / 总结 / 评论"扩成"**一句话 → 结构化图文 / 视频
文章 plan**,并在 card-host / App Hub / Rinx 三种宿主下都能完整可用、可降级"。

---

## 状态基线(v0.1.0,2026-10-01)

| 屏 | 主要控件 | 主要动作 |
| --- | --- | --- |
| 1 作品库 | 卡片 + `新建作品` | 进入第 2 屏 / 打开已有作品 |
| 2 录入 | 模式选择、URL 输入、原文输入、`进入创作` | 跳 Studio 屏 |
| 3 创作工坊 | 风格、参考图、原文预览、`AI 创作`、成稿输入、保存 | `octos.turn.start` 改写 / 翻译 / 总结 / 评论 |
| 4 导出 | 标题 + 成稿 + 参考图说明(只读) | 复制到外部平台 |

数据 `works.json`:`{id, title, source, reference, content, style, updated(seq)}`。

`manifest.json` 当前 capabilities:`storage / images / web / octos.session.open / octos.turn.start`。

---

## 平台能力可用 / 暂缓一览(2026-09-30)

### 可用 —— 本路线图全部要落地

| 能力 | 来源 / PR | 应用层用法 |
| --- | --- | --- |
| `octos.session.open / turn.start / interrupt` | OctoSense#106 / #184 | 与 v0.1.0 一致,改写 / 翻译 / 总结 / 评论 |
| `model.complete` + `model.budget` | App-Hub#24 / OctoSense#95 | 一次性、按 schema 校验的模型调用,用于意图创作 plan |
| `glance.publish / withdraw / list` | OctoSense#72 / #86 + App-Hub#22 | 把"今日灵感 / 推荐作品"发到 glance 屏 |
| `images` | 既有 | `Image{src: http_resource(url)}` 渲染已回填的 `image_url` |
| `storage / net / web / camera / microphone / library / location` | 既有 | 视后续需要启用,本路线图暂不加新 capability |
| `tools.json` 声明,被 Shell 注册到应用 peer | OctoSense#145 / #184 | 声明 OctoStudio 自有工具供 Agent 调用(`implemented_by: host-service` 才能落地) |
| `agent` 字段(准入检查可用) | App-Hub#18 | 声明应用 Agent 角色;`AGENT.md` / skills / needs / triggers **暂不装进 peer** |
| `card-studio / card-host --remote` | App-Hub#19 | 给开发者渲染 / 评审 L0 卡片,不入应用 |

### 暂缓 —— 等平台跟进,本路线图不预先动工

| 能力 | 当前状态 | 跟踪 |
| --- | --- | --- |
| `octos.*` 的 image / video output | 不存在 | 待 OctoSense 主仓新增 |
| `model.*` 的 image / video generation | 不存在 | 同上 |
| 视频显示 widget | SCRIPT-API 没有 | OctoScript-Makepad 跟进 |
| 应用触发器 / 后台运行 / 应用记忆 | 即将推出(ADR 0002 M3 / M7 / M8) | OctoSense #64 等 |
| 工具箱给应用 Agent(`toolbox-peers`) | 即将推出,随附 Shell 关闭构建特性 | OctoSense #64 |
| `sys.digest` L0 数据源 | 即将推出 | OctoScript#40 / OctoScript-Makepad#50 / OctoSense#87 |
| `model.complete` 直接返回 URL | 默认拒绝,需要 `allow_urls: true` + 模型自己产 URL | 现实是模型不知道 OctoStudio 的图床,本路线图不用 |
| `implemented_by: "app"` 的工具 | 当前被 Shell 拒("open the app to use it") | OctoSense `crates/shell/src/host_tools/script_apps.rs` |
| `AGENT.md` / `skills/` 装进 peer | 即将推出 | App-Hub#18 |

---

## 里程碑

### M0 — v0.1.1 修 v0.1.0 硬伤(半天,不重签) — ✅ 与 v0.2.0 合并落地

**目标** 修复 v0.1.0 实际可测到的 bug,小幅增量。

| 改动 | 文件 | 验证 |
| --- | --- | --- |
| AI 创作后把结果写回 `content_input` 并 render | `bundle/main.splash` `ask_ai()` | 录原文 → 点 AI 创作 → 不切屏即可见成稿 |
| 风格按钮闭包改用索引 `["改写","翻译","总结","评论"][i]` 替代循环捕获 | `bundle/main.splash` Studio 屏 chip 行 | 反复点四个风格按钮,激活态正确 |
| 参考图说明显隐与 Studio / Export 屏同步 | `bundle/main.splash` `goto()` 内的 reference_caption | 录合法 URL 后能看到"参考图"区 |
| Compose 屏 URL 输入即时 https 校验 | `compose_view` | 录 `ftp://` 不被静默接受 |
| README / REVIEW-ANSWERS 与实现同步 | `README.md`, `REVIEW-ANSWERS.md` | 文档一致 |

**实际状况:** v0.2.0 提交时一并修复了 AI 回写 + 风格按钮索引化 + 参考图显隐同步;URL 前缀即时校验未单独落地(在 card-host 中靠 `http_resource()` 的媒体规则天然拒 `ftp://`)。完整 check 通过。

完成后 `tools/octo check bundle/` 必须 PASSED,重抓截图,提交 v0.1.1 修复 commit
(README 第 300 行:签名之后任何修改都要重新 stamp + 签名 —— **本步不重签**,仅 stamp,
真正重签由发布者完成)。

### M1 — v0.2.0 意图创作(`model.complete`)(1–2 天,需重签) — ✅ 已落地

**目标** 一句话产出图文 / 视频文章 plan;真实生图 / 生视频由用户在外部工具完成,
应用只承担"结构化计划 + 复制 prompt + 导出"。

| 改动 | 文件 | 验证 |
| --- | --- | --- |
| `manifest.json` capabilities 加 `"model"` | `bundle/manifest.json` | ✅ `hub check` 接受;`grants: capabilities {..., "model"}` |
| `works.json` 追加可选字段 `prompt / plan_kind / plan_title / plan_sections / plan_scenes` | `bundle/main.splash` `load_works` 标准化 + `upsert_and_save` | ✅ 旧作品打开不报错(normalize 用 `+= {k:v}` 补齐) |
| Compose 屏加三个 chip:[粘贴原文] / [图文意图] / [视频意图];点后程序控制 visible 切换输入框 | `compose_view` | ✅ 截图 `06-intent-compose.png` |
| 新增第 4 / 5 屏(`intent_article_view`, `intent_video_view`),渲染图文 plan / 视频分镜 | `intent_article_view`, `intent_video_view` | ✅ 截图 `07-intent-article.png`, `08-intent-video.png` |
| `model.complete` 调用,带强制 schema(图文 / 视频 各一份),`class: "strong"`,`allow_urls: false` | `bundle/main.splash` `ask_intent_plan()` | ✅ schema 合规;真实模型返回时落 plan |
| UI 上每个 `image_prompt` / `video_prompt` 显示文本,文案"可全选复制" | `intent_article_view` / `intent_video_view` | ✅ 截图能看到 prompt 块 |
| Export 屏多格式导出:Markdown / 公众号 / Notion 三种 | `export_view` + `render_plan_as_text()` | ✅ 截图 `04-export.png` + `09-export-formats.png` |
| `listing.json` 描述 + `release_notes` 更新为 v0.2.0 | `bundle/listing.json` | ✅ 已更新 |
| `REVIEW-ANSWERS.md` Q3 / Q7 / Changelog 更新 | `REVIEW-ANSWERS.md` | ✅ 已更新 |
| `tools/octo check bundle/` + stamp | 仓库根 | ✅ PASSED(`unsigned` warning 由发布者重签) |

**降级文案(实际落地)**:

| 环境 | 显示 |
| --- | --- |
| `card-host` model 不可用 | "model 不可用,填入演示大纲 · N 段" / "填入演示分镜 · N 镜",plan 用预置 demo 数据填入 |
| Shell 未配 provider | `no_provider: <sentence>`,原样显示在 status |
| 配过 provider 但超预算 | `budget: <sentence>`,显示在 status |
| 模型返回不符合 schema | `invalid_output: <sentence>`,显示在 status |
| 用户未允许 agent(若用 `octos.*`) | `Waiting for the person to allow this app's agent (OctoSense asks the first time)` |

### M2 — v0.2.x 创作者工作流补齐(不依赖 AI,1 天)

**目标** 把 v0.1 缺的"基本功"补上,使用体验闭环。

| 功能 | 改法 | 验证 |
| --- | --- | --- |
| 删除作品 | 卡片长按弹确认 → `works.remove(...)` + `save_works()` | 长按卡片,看到"删除?"确认 |
| 字数统计 | Studio 屏底部状态条 `原文 N 字 · 成稿 M 字`;Export 屏同 | 截图能看出字数 |
| 多格式导出 | 三个按钮:Markdown / 公众号 / Notion;同一份 content 经纯函数转换 | 截图 `04-export.png` 重抓 |
| 参考图即时预览 | Compose 屏 URL 输入旁 `Image{src: …}`(64×64 缩略图);错误时占位 | 录合法 URL 后立即看到缩略图 |
| 相对时间显示 | `now - stamp` 估算:"刚刚 / N 分钟前 / N 小时前 / N 天前" | 截图 `01-home.png` 重抓 |
| AI 失败重试按钮 | "助手不可用"状态下显 `重试`,重发一次 `octos.session.open` | 截图 `05-ai-error.png` 改名为 `05-ai-retry.png` 重抓 |
| v0.1 风格按钮 closure 修复(M0 已做的不重复) | — | — |

### M3 — v0.3.0 内容管理(不依赖 AI,半天到 1 天)

> **2026-10-04 状态:场景广场(9 生成场景 + 通用编辑器 + 主题样式 + 制作包/Marp/markmap 导出)已在 v0.3.0 落地**;本节其余条目(搜索/排序/批量删/撤销/标签)仍待做。

| 功能 | 备注 |
| --- | --- |
| 搜索(按 title / source 子串) | 顶部加 `TextInput` + `on_change` 即时过滤 works_list |
| 排序(按 updated 倒序 / 正序 / 按 style) | 顶部 chip 切换,改变 `works_list` 渲染顺序 |
| 多选批量删除 | 卡片进入"多选模式"(checkbox),底部出操作条 |
| 撤销栈(最近 5 步 content_input 文本快照) | `let undo_stack = []; undo_stack.push(snapshot)`,最多 5 |
| 标签 `tags: ["分享","实战"，"测评"，"记录"]` | 存 `works.json`;UI 在卡片下显示 |
| AI 历史(每篇作品最近 3 条 `{prompt, result}`) | 存 `works.json.history`;Studio 屏"查看历史"展开 |
| 一键"全选 + 复制"提示 | 沿用 v0.1 选区复制,加"全选"快捷手势(在 Export 屏里) |

### M4 — v0.4.0 `model.complete` 高阶用法(1 天)

| 功能 | schema |
| --- | --- |
| 自动起标题 | `{title: string maxLength: 40}` |
| 关键词提取 | `{keywords: string[] maxItems: 5}` |
| 自动摘要(为长原文生成 100 字以内摘要) | `{summary: string maxLength: 200}` |
| 风格迁移(同一份原文,生成 4 种风格的对照稿) | `{variants: {style: string, body: string}[4]}` |
| 中英对照翻译 | `{translation: {en: string, zh: string}}` |
| 标题打分(给已有标题打 0–100) | `{score: integer, suggestions: string[]}` |
| 模型预算展示 | 进入 Studio / 创作意图时调一次 `model.budget`,显示今日剩余 |

全部走 `class: "fast"`,保持 6/min 与每日预算;schema 全部强制。降级文案沿用 M1。

### M5 — v0.5.0 glance 卡片(半天)

| 功能 | 改法 |
| --- | --- |
| 启用 `glance` capability | `manifest.json` capabilities 加 `"glance"` |
| 发布"今日灵感" | 进入应用时若有最新一篇未发布作品,提示"发送到 glance?" |
| 发布"今日推荐作品" | Home 屏每张卡片菜单加"发到 glance" |
| `glance.list` 在 Home 屏显示"已发布的卡片"列表 + 撤回按钮 | — |
| L0 卡片源(本地字符串,不用 `sys.digest`) | `bundle/assets/glance-today.card`(纯 L0 文本,引用 `data` 字段) |
| 降级文案 | `card-host` 显示 `no service answers "glance"`,按钮置灰 |
| 截图 | `09-glance-publish.png`,`10-glance-degraded.png` |

### M6 — v0.6.0 应用 Agent + `tools.json`(准入检查可用,实际收益有限)

**说明** `agent` 字段在 App Hub `main` 准入检查通过(已在 OctoSense Shell 中可用),
但 `implemented_by: "app"` 当前被 Shell 拒;真正可用的是 `host-service` 工具,而平台
目前没有给应用用的"工作类"工具。本里程碑做**声明层 + 文档**,实际执行留给平台跟进。

| 改动 | 文件 |
| --- | --- |
| `manifest.json` 加 `agent: { profile: "workspace-write", tools: ["work.save", "work.list", "work.delete"] }` | `bundle/manifest.json` |
| 写 `bundle/tools.json`,工具声明 `implemented_by: "host-service"`(等平台 host-service 实现) | `bundle/tools.json` |
| 写 `bundle/AGENT.md`,描述 OctoStudio 的 Agent 角色(只读 / 改写 / 计划) | `bundle/AGENT.md` |
| `skills/` 目录先建空壳(被准入检查接受,但不装进 peer) | `bundle/skills/` |
| `REVIEW-ANSWERS.md` Q3 列出 agent / tools 的当前状态 | `REVIEW-ANSWERS.md` |

### M7 — 等平台(本路线图不预先动工)

| 平台能力 | 跟踪 |
| --- | --- |
| `octos.*` 的 image / video output | OctoSense 主仓 |
| `model.*` 的 image / video generation | OctoSense 主仓 |
| 视频显示 widget | OctoScript-Makepad |
| 应用触发器 / 后台运行 / 应用记忆 | OctoSense #64 / ADR 0002 |
| 工具箱给应用 Agent(`toolbox-peers`) | OctoSense #64 |
| `sys.digest` L0 数据源 | OctoScript#40 / OctoScript-Makepad#50 / OctoSense#87 |
| `implemented_by: "app"` 工具真正执行 | OctoSense `crates/shell/src/host_tools/script_apps.rs` |
| `AGENT.md` / `skills/` 装进 peer | App-Hub#18 |

等任一项合入后,OctoStudio 即可以无改 manifest 地承接:把 `image_url / video_url` 从
"留空 + 占位"切到"真接 URL 渲染";视频 widget 上线后即可在 Studio / Export 屏做内嵌预览。

---

## 红线(全路线图遵守)

1. **不申请 `llm / news / mail / profile` 等"仅系统应用"的 capability** —— 准入检查
   接受,但对应服务在 `os.*` 之外一律返回空,商店会问"界面用不到的授权"。
2. **不收集密码 / PIN / 一次性码** —— `is_password: true` 字段会被准入检查拒。
3. **`network.hosts` 仍保持空数组** —— 所有 https 资源走 `images`(公网公开)或用户回填
   的 `image_url`;`net` capability 本路线图不申请。
4. **不写 `os.*` id** —— 这是商店应用,不是系统应用。
5. **不在 bundle 中放 token / API key / 私钥** —— hub scan 抓出即拒。
6. **截图必真实** —— 不放 dummy;截图前必须 `tools/octo check` PASSED。

---

## 验收与发布(每个里程碑都走一遍)

```sh
cd /Users/lijing/CodeProjects/OctoScript-App-Design-Flow
tools/octo doctor            # 全 [ok]
tools/octo run /Users/lijing/CodeProjects/octostudio/bundle --port 8142 --hidden --detach
# 交互测试(走 /click, /t, /snap, /g)
tools/octo shot 8142 /Users/lijing/CodeProjects/octostudio/bundle/screenshots/<新截图>.png
curl -s 127.0.0.1:8142/quit
tools/octo check /Users/lijing/CodeProjects/octostudio/bundle    # 必须 PASSED
# 提交(若是 v0.2.0 / v0.3.0 等,需要重签:)
hub stamp /Users/lijing/CodeProjects/octostudio/bundle
hub sign-manifest /Users/lijing/CodeProjects/octostudio/bundle \
    --key /secure/path/publisher.key --key-id amosgeek
hub check /Users/lijing/CodeProjects/octostudio/bundle \
    --publisher-key amosgeek=$(hub pubkey /secure/path/publisher.key)
```

---

## 风险与备注

- **M1 必须重签** —— v0.2.0 改了 `manifest.json` 的 `capabilities` 与 `bundle_blake3`,
  原 v0.1.0 的 amosgeek 签名即失效。签名步骤由发布者完成,我不跑。
- **M1 的 `model` 配额** —— 用户每日 100 calls / 100K tokens / 每分钟 6 calls,
  应用内要在调用前先 `model.budget` 看一眼,不要把用户配额一次打空。
- **M5 glance 的应用配额** —— 每分钟最多 6 次发布,每个应用 4 张卡,存储共 32 张,
  显示 6 张。UI 上要做"已发几张"的指示。
- **M6 当前收益有限** —— 用户允许 agent 后,`implemented_by: app` 工具会被拒;
  真正执行要等平台 host-service 工具上线,所以 M6 主要是"声明 + 文档",先把
  manifest 跑通。
- **截图真实** —— `--hidden` 下抓的也算真实,因为是应用自身渲染管线出图(README 第 194 行)。
  不要用 `sips` / `convert` 合成。
- **重抓时一律加 `--settle 2`**(README 第 313 行)以等输入框渲染完成。
