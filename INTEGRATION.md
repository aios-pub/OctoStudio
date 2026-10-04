# OctoStudio ↔ OctoSense 接入指南(本机实测)

> 2026-10-04 实测通过:OctoStudio v0.3.0 已安装进本机 OctoSense 桌面壳,`model.complete`
> 经内置 AI 助手(agnes-3.0-flash provider)真实生成,`octos.*` 服务就绪。
> 本文档记录接入机制、一次性准备(已完成)、日常启动命令与更新流程。

## 接入机制(为什么这样做)

- OctoSense 桌面壳(`/Volumes/PSSD/CodeProjects/OctoSense`,包 `octosense`)通过内嵌的
  **card runner** 运行脚本应用;应用发现走 `$OCTOSENSE_APP_DATA`(本机即 `~/.octosense/apps`)。
- **手放 bundle 到 apps 目录不够**:打开已装应用要过 App Hub 的 catalog 账本
  (`may_open` → `store.may_run`),必须走一次真实"发布 → 安装"。
- 本地发布用 PUBLISHING.md 的**本地彩排路径**:自己的 anchor key 签一个本地 mirror 目录,
  壳用 `OCTOSENSE_HUB`(mirror 路径)+ `OCTOSENSE_HUB_ANCHOR`(本地 anchor 公钥)启动,
  然后在壳内 App Hub 里 Get → Install → Open。
- AI 助手链路:`octos-kernel`(exec,`kernel-artifact.py --host` 产出)+ `os.ai-providers`
  的 provider profile(`~/.octosense/octos-home/.octos/profiles/_main.json`,agnes-3.0-flash,
  密钥在钥匙串)。`model.complete` 按 manifest 的 `model` 能力授权、每日配额。

## 一次性准备(本机已完成,勿重复)

```sh
OCTO=/Volumes/PSSD/CodeProjects/OctoSense          # OctoSense 仓库
APP=/Volumes/PSSD/CodeProjects/octostudio           # 本应用
HUB=/Volumes/PSSD/dev/rust-target/release/hub       # hub CLI(App Hub 仓库构建)
KEYS=$APP/build/keys; M=$APP/build/mirror           # 都在 gitignore 的 build/ 下

# 1) 壳与内核(约 10min + 32min;已完成)
cd $OCTO && CARGO_TARGET_DIR=/Volumes/PSSD/dev/rust-target cargo build --release -p octosense
CARGO_TARGET_DIR=/Volumes/PSSD/dev/rust-target python3 tools/kernel-artifact.py \
  --host --stage /Volumes/PSSD/dev/rust-target/release   # octos-kernel + receipt

# 2) 本地 anchor 与 mirror 发布(已完成;anchor 公钥存 build/anchor.hex)
ANCHOR=$($HUB keygen $KEYS/anchor.key)
$HUB keygen $KEYS/working.key > /dev/null
CERT=$($HUB certify --anchor $KEYS/anchor.key --working $KEYS/working.key)
$HUB sign-manifest $APP/bundle --key ~/.octosense/amosarc-publisher.key --key-id amosarc
$HUB publish $APP/bundle --catalog $M/catalog.json --key $KEYS/working.key \
  --anchor-cert "$CERT" --publisher amosarc \
  --publisher-key "amosarc=$($HUB pubkey ~/.octosense/amosarc-publisher.key)" \
  --repo https://github.com/aios-pub/OctoStudio.git \
  --commit "$(git -C $APP rev-parse HEAD)" --out $M
$HUB verify $M/catalog.json --anchor "$ANCHOR"    # → catalog sequence 1 verified

# 3) 壳内安装(已完成):带环境变量启动 → dock 打开 App Hub → OctoStudio → Get → Install → Open
#    结果:~/.octosense/apps/octostudio/bundle/(App Hub 安装)+ catalog.json(sequence 1)
```

## 日常启动(GUI)

```sh
cd /Volumes/PSSD/CodeProjects/OctoSense
OCTOSENSE_HUB=/Volumes/PSSD/CodeProjects/octostudio/build/mirror \
OCTOSENSE_HUB_ANCHOR="$(cat /Volumes/PSSD/CodeProjects/octostudio/build/anchor.hex)" \
/Volumes/PSSD/dev/rust-target/release/octosense
```

- 启动器/dock 里打开 **OctoStudio**(已安装,重启不丢)。
- 两个环境变量**必须带**:catalog 是本地 anchor 签的,默认只信线上 App Hub anchor,
  不带时 App Hub 列表与已装应用都会被拒(stock build 只信官方 anchor)。
- 首次在 OctoStudio 里用 octos 二创时,壳会弹 **agent 授权面板**(这是正常的首用同意,
  点允许);`model` 权限在安装时已经确认过。
- 无头驱动(自动化测试)加 `MAKEPAD_HIDE_WINDOWS=1 MAKEPAD_REMOTE=<port>`,跳过授权面板可加
  `OCTOSENSE_CONTAINED_APPS=1`(仅开发用)。

## 更新应用(改了 bundle 之后)

```sh
cd $APP && <改 bundle/main.splash 等>
OCTO=/Volumes/PSSD/CodeProjects/OctoScript-App-Design-Flow/tools/octo && $OCTO check bundle
$HUB sign-manifest bundle --key ~/.octosense/amosarc-publisher.key --key-id amosarc
$HUB publish bundle --catalog $M/catalog.json --key $KEYS/working.key \
  --anchor-cert "$CERT" --publisher amosarc \
  --publisher-key "amosarc=$($HUB pubkey ~/.octosense/amosarc-publisher.key)" \
  --repo https://github.com/aios-pub/OctoStudio.git --commit "$(git rev-parse HEAD)" --out $M
# 重启壳(带上面的环境变量)→ App Hub 里会出现新版本 → Get 安装
```

## 已验证(2026-10-04,本机 macOS)

- `hub publish` → `catalog sequence 1 verified, 1 entries`
- 壳内 App Hub 列出 OctoStudio → 权限摘要六项(含"Send … to the AI provider you
  configured")→ Install → `Installed. Your app is ready to open.` → Open
- `card: octostudio running under 6 capability(ies), 1 host(s)`(card runner 沙箱)
- 应用状态栏 `octos + model 已就绪`;输入"冬日胡同咖啡香" → **真实 model.complete 调用**
  (非 demo 降级),生成 4 段咖啡主题图文 + 英文生图 prompt,AI 配图(pollinations)加载
- 数据落盘 jail:`~/.octosense/apps/octostudio/`(accounts/cache/common)
- 不带 `OCTOSENSE_HUB(_ANCHOR)` 启动时已装应用被拒(预期:本地彩排 anchor ≠ 官方 anchor)

## 正式上线后

提交 App Hub issue(人工关卡)并通过审核、进入官方 catalog 后,任何 stock 壳都能直接安装,
本地 mirror 与环境变量即可弃用。
