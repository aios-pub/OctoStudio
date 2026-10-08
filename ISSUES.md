# 我向 OctoSense 生态提的 issue 清单

> 维护者:Amos · 仓库:OctoStudio · 范围:OctoSense-App-Hub / OctoSense / Rinx 三个相关项目
>
> 全部 issue 由本人在 OctoStudio v0.4.x 开发过程中,基于真实 host-service 探查、card-host probe、cargo 集成测试发现的能力/互操作缺口提交。

## 一、OctoSense-App-Hub(共 5 个)

### #73 — OctoStudio v0.4.3 提交

- **仓库**:[OctoSense-org/OctoSense-App-Hub#73](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/73)
- **类型**:App submission(初赛提交)
- **状态**:⏳ 阻塞:`publisher key "aios.pub" is not registered with this hub`(需 App Hub 评审人工注册)
- **内容要点**(仅必要信息,无冗余):
  - App / 仓库地址 / tag(`octostudio-0.4.3`,无 `.aios.pub` 后缀)/ Bundle 路径
  - Publisher key 标识:`aios.pub`
  - `hub check` gate 输出
  - What's new(v0.4.2 → v0.4.3 修复)
  - Publisher 联系信息
- **关联**:本仓库 `bundle/main.splash` v0.4.3、tag `octostudio-0.4.3` 已推送到 `aios-pub/OctoStudio`

### #85 — Add `model.image` host service(文生图)

- **仓库**:[OctoSense-org/OctoSense-App-Hub#85](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/85)
- **类型**:平台能力缺口
- **状态**:❌ 未发布(文档从未列名)
- **发现方式**:card-host probe 返回 `no service answers "model" on this device`,且源码 `grep -hoE '"model\.[a-z_.]+"' apps/ai-providers/host-service/src/ -r | sort -u` 唯一输出 `"model.complete"`
- **诉求**:在 `model` family 下新增 `model.image` 服务,与 `model.complete` 对齐 schema 校验、budget 计费、URL 审查

### #86 — Add `model.video` host service(文生视频)

- **仓库**:[OctoSense-org/OctoSense-App-Hub#86](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/86)
- **类型**:平台能力缺口
- **状态**:❌ 未发布
- **发现方式**:同 #85(`grep` 零结果)
- **诉求**:在 `model` family 下新增 `model.video`,支持文本/参考图 → 视频

### #87 — Add `model.audio` host service(TTS)

- **仓库**:[OctoSense-org/OctoSense-App-Hub#87](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/87)
- **类型**:平台能力缺口
- **状态**:❌ 未发布
- **发现方式**:同 #85
- **诉求**:文字转语音 host service

### #88 — Add `model.embeddings` host service(嵌入)

- **仓库**:[OctoSense-org/OctoSense-App-Hub#88](https://github.com/OctoSense-org/OctoSense-App-Hub/issues/88)
- **类型**:平台能力缺口
- **状态**:❌ 未发布
- **发现方式**:同 #85
- **诉求**:文本向量嵌入,供检索/RAG/相似度场景

## 二、OctoSense(共 1 个 umbrella issue)

### #332 — Add `octos.image` / `octos.video` / `octos.audio` / `octos.embeddings`

- **仓库**:[OctoSense-org/OctoSense#332](https://github.com/OctoSense-org/OctoSense/issues/332)
- **类型**:平台能力缺口(Agent 端一次性提出 4 项)
- **状态**:❌ 未发布
- **发现方式**:
  - `grep -hoE '"octos\.[a-z_.]+"' crates/ai-host/src/ -r | sort -u` 实际只有 `octos.render` / `octos.session.open` / `octos.turn.start` / `octos.turn.interrupt` / `octos.session.history`
  - card-host probe 返回 `not granted "octos", which "octos.image" needs`(在 manifest 未声明时)
- **诉求**:在 `octos` family 下新增 4 个 Agent 触发方法(image / video / audio / embeddings),与 #85–#88 配套

### #380 — Desktop shell launcher popup 不显示已装 Card 应用

- **仓库**:[OctoSense-org/OctoSense#380](https://github.com/OctoSense-org/OctoSense/issues/380)
- **类型**:产品 bug(desktop shell 集成路径)
- **状态**:⏳ 待评审
- **现象**:OctoStudio 0.4.4 经本地 mirror 发布 + catalog sequence 5 admit + 装到 `~/.octosense/apps/.bundles/octostudio/bundle/`,`integrity.bundle_blake3` 与目录对齐,`may_run` 通过。**但桌面壳 launcher 弹窗(Apps drawer)只显示 9 个系统 card 应用(Reference/Browser/Files/Terminal/Task Manager/Sheets/Clock/Weather/Finance),不显示已装 OctoStudio;dock 同样漏掉。**
- **根因**(从 `crates/shell/src/apps.rs` + `crates/app-hub-app/src/lib.rs` + `crates/appstore/src/lib.rs` 读出):
  - `installed_apps()` 正确读到 `octostudio 0.4.4`
  - `apps_uncached()` 用 `clients::available_apps()` 合并 system + installed 后过 `is_launchable` 滤
  - `is_launchable()` 在 `hosting == Module` 分支要求 `registry.module(&app.id).is_some()`;installed app 的 id 是 `octostudio`,但 card runner 注册为 `card` 模块,不是 per-app 模块,lookup 失败 → `is_launchable` 返 false → 行被丢弃
  - `apps_uncached` 链未对 `hub:` 前缀的 id 放行
- **诊断依据**:
  - 同一根因下,`~/.octosense/apps/.running/*/` 仍有 12 份 v0.3.x 缓存(说明 installed-then-launched 路径历史可用)
  - 直接 `card-host --bundle .../bundle --app-data ~/.octosense --allow-unsigned` 跑得通(独立模式),证明 bundle 本身无问题
  - 阻塞只发生在 `desktop shell` 的 launcher 与 dock 路径
- **影响**:用户从 App Hub 装的所有脚本应用,在桌面壳里都不可见、点不到。手机/桌面的端用户实际拿不到已装的应用,App Hub 商店的真实价值被阻断。
- **建议修复**:
  1. `apps::is_launchable` 对 `hosting == Module && id.starts_with("hub:")` 直接放行(card 模块托管所有脚本应用,`module_open` 已能正确路由)
  2. `apps_uncached` 在 `is_launchable` 过滤前对 `hub:` 前缀的 installed 行豁免
- **临时绕过**:开发者本地 `card-host` 直跑,或用户在 App Hub 窗口里点 *Open*。
- **关联**:与 [[#327]] 一起读(同属 desktop shell 与 card-host isolate 之间的 host.service pump 缺口群)

## 三、hagency-org/Rinx(共 1 个)

### #63 — card mini-app `octos.*` 异步回复缺口

- **仓库**:[hagency-org/Rinx#63](https://github.com/hagency-org/Rinx/issues/63)
- **类型**:跨平台互操作 / host.request 异步回复
- **状态**:⏳ 平台侧 host.request 异步回复未接线
- **现象**:card mini-app 场景中,`host.request` 的异步回复(slow `model.complete` 返回的 Result)从未到达应用回调;同步调用正常
- **诊断依据**:
  - probe 对照诊断法(同一调用在 OctoSense card-host 返回 OK,在 Rinx card mini-app 返回空)
  - 应用侧 70s 看门狗兜底(避免 UI 永久悬挂)
- **影响**:任何「小说 / 拆解视频 / 长文章」等会触发 `model.complete` 长延迟的创作场景在 Rinx 端都不能拿到最终结果
- **临时绕过**:应用侧加 70s 超时 + UI 显示「等待结果超时,请稍后手动重试」;OctoStudio 已采用

## 统计

| 仓库 | 我提的 issue 数 | 其中能力缺口 | 其中提交/互操作 |
|---|---|---|---|
| OctoSense-App-Hub | 5 | 4 | 1 |
| OctoSense | 2 | 1 | 1 |
| Rinx | 1 | 0 | 1 |
| **合计** | **8** | **5** | **3** |

补充评论:
- **#327**(OctoSense):本轮在 main @ 2026-10-08 + OctoStudio 0.4.4 + 70s 应用侧看门狗下完整复现(`model.budget` 同步返 OK,`model.complete` worker-thread 回调永不触发,ledger 显示调用已记账)。Reporter 的 `take_replies_for` 泵修复方向正确,从 `crates/shell/src/module_host.rs` 也能看到 card isolate 路径确实没有接这个 pump。