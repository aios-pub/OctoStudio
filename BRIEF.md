# OctoStudio

黑客松参赛项目 — 基于 OctoScript App Design Flow 的脚本应用,同一份 `bundle/`
同时投递 Rinx mini-app 与 OctoSense App Hub,**面向创作**(内容创作、写作、参考资料管理)。

> 注意:本仓库项目文档约定项目文档约束不为 Office 文档;此文件名为 BRIEF.md 仅为
> `flows/script-app/FLOW.md` 第 1 步要求的产物。

## 一句话

让创作者从一段原文 / 一段英文 / 一组参考素材出发,在本地保存作品、调用设备助手做改写与翻译、把成稿复制到外部平台(微信公众号、Markdown 编辑器、Notion 等)。
在 Rinx 中可调用设备 AI,在 OctoSense 商店与独立 `card-host` 中 AI 列为"未验证"。

## 屏幕(共 4 个)

| # | 屏幕 | 主要控件 | 主要动作 |
| --- | --- | --- | --- |
| 1 | 作品库(Home) | 作品卡片(标题 + 风格 + 封面缩略图)、`新建作品` 按钮 | 进入第 2 屏 / 打开已有作品 |
| 2 | 录入(Compose) | 模式选择(粘贴文本 / 上传参考 URL)、参考图 URL、文本输入、`进入创作` 按钮 | 抓取 URL 内容或填入原文 |
| 3 | 创作工坊(Studio) | 风格下拉(改写 / 翻译 / 总结 / 评论)、参考图预览、`AI 创作` 按钮、AI 状态区、当前作品预览(可手动改) | 调用 `octos.turn.start` 创作 |
| 4 | 导出(Export) | 标题预览、参考图预览、成稿预览(全选可复制) | 复制到外部平台 |

## 主要动作

| 动作 | 触发 | 期望 |
|---|---|---|
| 新建作品 | Home → `新建作品` | 跳转 Compose 屏 |
| 录入原文 | Compose 屏 → 文本框 + `进入创作` | 跳 Studio 屏 |
| 添加参考图 | Compose 屏 → 参考图 URL 输入 | 在 Studio 屏渲染缩略图(走 `images` 能力) |
| AI 创作 | Studio 屏 → 选风格 + `AI 创作` | `octos.turn.start` 返回译文/改写,落屏 |
| 保存作品 | Studio 屏 → `保存` | 写入 `storage`,跳回 Home |
| 打开作品 | Home → 卡片点击 | 跳 Studio 屏恢复 |
| 复制成稿 | Export 屏 → 选中文本 → 系统复制 | 粘贴到目标平台 |

## 数据

`storage` 能力下的 `drafts.json`:

| 字段 | 类型 | 说明 |
|---|---|---|
| `id` | string | 时间戳 id |
| `title` | string | 作品标题 |
| `source` | string | 原文 / 原始素材 |
| `reference` | string? | 参考图 URL(`https://`) |
| `content` | string | 成稿(AI 生成或手动) |
| `style` | string | 改写 / 翻译 / 总结 / 评论 |
| `updated` | number | 最近一次保存时间(epoch ms) |

不存密钥、token、API key;不存用户账号信息。

## 申请的 capability(逐条理由)

| 能力 | 用途 | 屏/动作 |
|---|---|---|
| `storage` | 存作品 | 1, 3, 4 |
| `images` | 显示参考图(`https://`,只读) | 3, 4 |
| `web` | 打开公开 https 网页(可选,用于参考阅读) | 文档中说明,本 v0.1 未主动调用 |
| `octos.session.open` | 设备 AI peer(在 Rinx 中可用) | 3 |
| `octos.turn.start` | 创作改写/翻译(在 Rinx 中可用) | 3 |

不申请 `net`(无特定主机列表)、`camera`(本 v0.1 不截图)、`library`、`mail`、`model`、`llm`、
`news`、`glance`、`location` 等,本 v0.1 不涉及。

## 状态(必测)

| 状态 | 屏 | 期望 |
|---|---|---|
| 作品库为空 | 1 | "还没有作品,点新建开始一段创作。" |
| AI 不可用(card-host / App Hub) | 3 | "助手不可用,可手动创作";UI 继续可写 |
| AI 正在生成 | 3 | `AI 创作` 按钮禁用,显示"等待助手…" |
| 参考图为空 | 3, 4 | 隐藏参考图区域,不显示破图 |
| 参考图 URL 错误 | 3, 4 | 占位符或友好提示,不崩溃 |

## 边界 / 不做

- **不接微信 OAuth/secret**:微信发布由人到后台粘贴,凭据永不入包。
- **不存密码/Token**:应用不调用 `mail.*`,不会要账号。
- **不写 `os.*` id**:这是商店应用,不是系统应用。
- **不做视频生成**:当前 `octos.*` 与 `model` 均无视频能力。

## 双渠道投递

| 渠道 | AI 创作 | 参考图 | 存作品 | 复制 |
|---|---|---|---|---|
| Rinx(默认目标) | 可用 | 可用 | 可用 | 选区复制 |
| OctoSense App Hub / 独立 card-host | 不可用(列未验证) | 可用 | 可用 | 选区复制 |

清单中如实写明。

## 出包与验证

- 步骤 4 起按 `flows/script-app/FLOW.md` 执行。
- `tools/octo check bundle/` 必须 `— PASSED`。
- 截图在 `bundle/screenshots/01-home.png` 等,以 `listing.json` 中的名字为准。
- 提交前 hub scan 的 7 个问题由我代写,签名前停下交人。