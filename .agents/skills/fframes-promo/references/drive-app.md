# Driving the OctoStudio app via the card-host remote bridge

When you need to recapture a real screenshot (PNG got overwritten, content changed, app behavior moved), drive the app headlessly and save the frame.

## Launch

```sh
# Build dev-bundle by stripping the publisher signature (card-host refuses signed bundles)
rm -rf build/dev-bundle && mkdir -p build/dev-bundle
cp -R bundle/assets bundle/main.splash bundle/manifest.json build/dev-bundle/
python3 -c "
import json
m = json.load(open('build/dev-bundle/manifest.json'))
m['integrity'].pop('signature', None)
json.dump(m, open('build/dev-bundle/manifest.json','w'), indent=2, ensure_ascii=False)
"

# Set card-host path if it's not on PATH
export OCTO_CARD_HOST=/Volumes/PSSD/dev/rust-target/release/card-host

# Launch detached on a free port
OCTO_CARD_HOST=$OCTO_CARD_HOST tools/octo run build/dev-bundle \
    --port 8143 --hidden --detach
# Output: "ready: first frame drawn" "pid NNNN"
```

`--hidden` keeps card-host from grabbing screen focus. Multiple ports can coexist.

## Health check (3 commands, ~3s)

```sh
pgrep -fl card-host | head -1                                    # process exists
curl -s 127.0.0.1:8143/ | head -1                               # "makepad-remote ... windows=1"
tools/octo shot 8143 /tmp/octo_smoke.png && open /tmp/octo_smoke.png  # visible render
```

## Remote bridge routes (GET, CSS-point coords)

| Route | Effect |
|---|---|
| `/snap?q=<hint>` | JSON tree; rect + text per component |
| `/d` | Full component tree, text only |
| `/click?x=&y=&wait=1` | Click at CSS point (412×803 window) |
| `/t?t=<text>&wait=1` | Type into focused input |
| `/k?k=down&c=ReturnKey` | One key event |
| `/log?n=50` | Last N log lines |
| `/g?raw=1` | Window PNG (same as `tools/octo shot`) |
| `/quit` or `/gq` | Quit (always call last) |

Coordinates are **CSS points**, not pixels. On retina, a 412×803 window is 824×1606 pixels but `/click` still takes 412-grid coords.

## Walk-through pattern (capture a scenario's plan page)

```sh
# 1. Find the card
curl -s "127.0.0.1:8143/snap?q=图文文章" > /tmp/snap.json
# Parse: Label [102, 266, 53, 45] '图文文章' → click center (128, 288)
curl -s "127.0.0.1:8143/click?x=128&y=288&wait=1" >/dev/null
sleep 1

# 2. Find the intent input
curl -s "127.0.0.1:8143/snap?q=一句话意图" > /tmp/snap.json
# TextInput [16, 561, 380, 80] → center (206, 601)
curl -s "127.0.0.1:8143/click?x=206&y=601&wait=1" >/dev/null

# 3. Type
curl -s "127.0.0.1:8143/t?t=夏日海边慢生活,治愈系图文&wait=1" >/dev/null
sleep 1

# 4. Find and click generate
curl -s "127.0.0.1:8143/snap?q=生成图文" > /tmp/snap.json
# Button [113, 744, 89, 40] → center (157, 764)
curl -s "127.0.0.1:8143/click?x=157&y=764&wait=1" >/dev/null

# 5. Wait for generation (≥6s for the 70s watchdog's slow-response generation)
sleep 6
curl -s "127.0.0.1:8143/snap?q=共" > /tmp/snap.json
# Verify '共 N 条 · 摘要 N 字' label exists

# 6. Capture
curl -s "127.0.0.1:8143/g?raw=1" -o promo/media/03-article-images.png

# 7. Clean up
curl -s "127.0.0.1:8143/quit" >/dev/null
```

## Parsing `/snap` for component rects

The output is JSON. Walk `d['s']` recursively and match on `t` (text content):

```python
import json, sys
d = json.load(open('/tmp/snap.json'))
def walk(n):
    t = n.get('t','')
    if isinstance(t, str) and '图文文章' in t and len(t) < 40:
        print(n.get('ty'), n.get('r'), t[:30])
    s = n.get('s')
    if isinstance(s, list):
        for c in s: walk(c)
for n in d.get('s', []): walk(n)
```

The `r` field is `[x, y, w, h]`. Click center is `(x + w/2, y + h/2)`.

## When generation falls back to demo content

If `model`/`octos.*` are not provided by the host (typical for shell integration), the app fills demo content automatically. You'll see `model 不可用...` in the status bar but plan items still get populated with placeholder content.

This is **fine** for promo screenshots — just match the typed intent to the demo content shown (e.g. type "夏日海边慢生活,治愈系图文" to get the beach-themed demo).

## Generation watchdog

The app has a 70-second watchdog (per memory): if `host.request` for `model.complete` exceeds 70s without callback, the busy flag resets so the user can retry. Bridge timeouts (60s default) may drop the async reply — that's the gap the watchdog covers.

For your capture script: wait at least 6-10 seconds after clicking generate before checking for plan items. If demo content shows up immediately, you can grab the screenshot right away.

## Desktop window for desktop screenshots

card-host defaults to a 412×803 (phone portrait) window. Use `--size WxH` to open desktop dimensions:

```sh
tools/octo run build/dev-bundle --port 8143 --hidden --detach --size 1280x800
```

The splash layout is single-column with Fill-width children, so it works either way. The `--size` is the official escape hatch from the no-breakpoint world.

## Common errors

- **`card-host: refused: no signature verifier`** — your bundle still has a publisher signature. Strip it (see Launch above) and rerun.
- **`card-host: refused: bundle digest does not match the manifest`** — your dev-bundle's manifest has the original stamp but the file bytes changed. Re-run with `--stamp` (default) so octo rewrites the digest, OR strip the manifest integrity and let card-host use the regenerated one.
- **`port already in use`** — another app or instance is on 8143. Pick a different port.
- **Bridge not responding** — pgrep showed card-host but `/` returns nothing. Wait 2-3 seconds after "ready: first frame drawn" before first request.
