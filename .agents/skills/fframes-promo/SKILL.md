---
name: fframes-promo
description: How to build, iterate on and ship a 1920x1080 / 30 fps promo video with the fframes crate (Rust, Skia Metal backend) — the format used in `promo/` of this repo. Use whenever the user mentions promo video, fframes scenes, `cargo run -- timeline/frame/render`, asks to add a scene, wants to wire BGM/SFX into AudioMap, drive the OctoStudio app via /click /t /g to recapture a screenshot, or hits fframes' borrow/lifetime/transform traps. Also load when bundling issues arise — `card-host: refused`, octo `octo run --stamp` rewriting a signed manifest, git 443 timeouts, `hub publish` failing with "anchor did not certify this working key".
---

# fframes promo: build, run, ship

The `promo/` crate in this repo is a video rendered by [fframes](https://github.com/dmtrKovalenko/fframes) (version `=1.2.0`, Skia Metal backend). The promo loop is:

```sh
cd promo
cargo run --release -- timeline            # scenes, durations, audio tracks
cargo run --release -- frame "<scene>@<t>" # PNGs into frames/ and problems found in them
cargo run --release -- inspect             # missing media, clipped text, panics, in every frame
cargo run --release -- audio analyze       # LUFS, peaks, silence per scene
cargo run --release -- render              # the final video, writes out.mp4
cargo test                                  # frame snapshots; FFRAMES_UPDATE_SNAPSHOTS=1 to accept
```

`times` accept frames (`120`), seconds (`3.2s`), `m:ss`, percentages (`50%`), scene names (`IntroScene`, `#1`) and offsets inside scenes (`IntroScene@1.5s`, `@50%`, `@end`). Add `--json` for machine-readable output.

## Project shape

- `src/lib.rs` — all scenes, the `Video` impl, mockup components, anim helpers
- `src/main.rs` — thin shim that runs `fframes::cli`
- `media/` — flat directory of fonts/images/audio embedded into the binary by `include_media_dir!`. **No subdirectories** — see pitfalls
- `tests/frames.rs` — visual regression; uses `snapshot::assert_frames` with scene names that match `Scene::name()` defaults

A 1920×1080 video is constant — don't try to vary it.

## Authoring a scene

Each scene is a `Scene` impl on a unit struct. The pattern that ships:

```rust
#[derive(Debug)]
struct FooScene;
impl Scene for FooScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let op = ramp(&frame, 0.3);              // 0→1 with cubic ease, defined in lib.rs
        let y  = rise(&frame, 0.5);              // 70→0 with spring
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
            transform={Transform::translate(0.0, y)}>
            ... children ...
        </g>)
    }
}
```

Register it in the video's `define_scenes` — order = playback order.

## Anim helpers (copy these into lib.rs)

`ramp`, `rise`, `slide_from_right`, `slide_from_left`, `scale_pop`, `kenburns`, `dip_in`, `dip_out` cover ~95% of entrance/loop animations. `wipe_parts(frame)` returns the BG wipe panels used for Flow/Constellation — call last so they sit on top.

### Transitions

- **Default**: dip-from-BG. Add `<rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />` as the last child so a BG-colored overlay fades out over 0.4s. For symmetry, use `dip_out(&frame, scene_len)` in the outro scene to fade to BG.
- **Wipe** (for chart/graph scenes): emit `wipe_parts(&frame)` at the end. It returns an empty `Vec` once the wipe is done (~0.55s), so it's safe to always include it.

Avoid cut-to-cut transitions — they feel amateur.

## AudioMap

```rust
fn audio(&self) -> AudioMap<'_> {
    AudioMap::from([
        AudioTrack::new("bgm60.wav", Second(0.)..Eof).gain_db(-5.0).fade_in(1.5).fade_out(2.5),
        AudioTrack::new("whoosh-sfx.wav", Second(6.85)..Eof).volume(1.4),
        AudioTrack::new("pop-sfx.wav",  Second(9.3)..Eof).volume(1.4),
        // ...
    ])
}
```

Absolute seconds. `volume(N)` is a linear scale (`1.0` = unity, `0.5` ≈ -6 dB). `gain_db(N)` for dB. BGM should sit at roughly -16 LUFS integrated after mixing sfx — verify with `cargo run --release -- audio analyze`.

Synthesize sfx with ffmpeg, not external libs:

```sh
# whoosh: brown noise + lowpass sweep + envelope (~0.7s)
ffmpeg -y -f lavfi -i "anoisesrc=colour=brown:d=0.7:amplitude=0.8" \
  -af "lowpass=f=900:poles=2,highpass=f=120,afade=t=in:d=0.18,afade=t=out:st=0.3:d=0.4,volume=1.4" \
  -ar 44100 -ac 2 whoosh-sfx.wav

# pop: short sine with echo decay (~0.2s)
ffmpeg -y -f lavfi -i "sine=frequency=620:d=0.14" \
  -af "volume=0.55,afade=t=out:st=0.02:d=0.12,aecho=0.6:0.3:60:0.25" \
  -ar 44100 -ac 2 pop-sfx.wav
```

Audio files **must be flat in `media/`** — `include_media_dir!` does not recurse.

## Validation loop (do every iteration)

1. `cargo check` — fix borrow/lifetime errors (see pitfalls)
2. `cargo run --release -- timeline` — confirm 60.000s and audio tracks resolved
3. `cargo run --release -- frame "<NewScene>@2.5s,<OtherScene>@3s"` — eyeball each new/edited scene's middle frame
4. `cargo run --release -- inspect` — must print `no problems found` for every frame
5. `cargo run --release -- audio analyze` — integrated LUFS in [-20, -14], true peak ≤ -1 dBTP, no clipped samples
6. `cargo test` (with `FFRAMES_UPDATE_SNAPSHOTS=1` on intentional visual changes)
7. `cargo run --release -- render` — final `out.mp4`. ffprobe to confirm duration + H.264 + AAC.

## Pitfalls (the things that always bite)

**svgr borrow trap** — `fframes::svgr!` returns an `Svgr` whose content lifetime borrows the literals you put inside. Local `String` or `format!()` results will fail with `cannot return value referencing local variable`:

```rust
// BAD — compiles fail
let badge = format!("0{}", i + 1);
fframes::svgr!(...{badge.as_str()}...)

// GOOD — static array
static BADGES: [&str; 10] = ["01","02","03","04","05","06","07","08","09","10"];
fframes::svgr!(...{BADGES[i]}...)
```

Same with `let label = format!(...)` inside a `(0..6).map(...)` — use `static SHOT_LABELS: [&str; 6] = [...]; SHOT_LABELS[i]`.

**Type ambiguity in clamp/percent** — `let p = animate(...); p.clamp(0.0, 1.0)` fails with `can't call method clamp on ambiguous numeric type {float}`. Annotate: `let p: f32 = animate(...);`.

**`fill={accent}` over `fill={**accent}`** — when iterating `&[(&str, &str)]`, the inner is already `&&str`. `From<&&str>` is implemented for the SVG value type; `From<str>` is not. Use `accent` directly.

**SVG `rotate()` takes degrees, not radians** — there is no radians mode. Use `-26.0 + i as f32 * 13.0` directly. `std::f32::consts::PI / 180.0` is only for computing trig of the angle.

**No gradients or filters** — fframes supports line/rect/circle/ellipse/polygon/polyline/path/text/image/g/svg and nothing more. To fake a glow, stack translucent filled shapes at descending opacities. Don't try `linearGradient` or `feGaussianBlur`.

**Hero scene with 3-card fan/arc — labels overlap** — a layout that looks fine in design tools overlaps when rotated cards share a pivot. Prefer horizontal pills with tiny rotation, or stack labels above each card. Verify with `frame` export.

**`<rect>` and `<line>` inside `<image>` group for Ken Burns** — `kenburns(frame)` returns scale 1.0→1.045 over 6s. Wrap the `<image>` in a `<g transform="translate(kx ky) scale(kb)">` where kx = w*(1-kb)/2. Static screenshots feel alive without distracting.

**`Scene::name()` defaults to type-name** — short-name form strips generics and `::`. So your scene struct `ScenarioGridScene` is referencable as `ScenarioGridScene@2.5s` in `frame`/`timeline`. If you change a struct's name, update `tests/frames.rs` to match.

**`octo run --stamp` invalidates signed manifests** — running `octo run` against `bundle/` after `hub sign-manifest` will rewrite `bundle/manifest.json`'s `bundle_blake3`, breaking the publisher signature. Recover with `git checkout bundle/manifest.json` (the manifest is in git, only the signature's payload hash changed).

**`card-host: refused: no signature verifier` for signed bundles** — local dev with a signed bundle needs a signature-stripped copy:

```sh
rm -rf build/dev-bundle && mkdir -p build/dev-bundle
cp -R bundle/assets bundle/main.splash bundle/manifest.json build/dev-bundle/
python3 -c "import json; m=json.load(open('build/dev-bundle/manifest.json')); m['integrity'].pop('signature', None); json.dump(m, open('build/dev-bundle/manifest.json','w'), indent=2, ensure_ascii=False)"
OCTO_CARD_HOST=/path/card-host tools/octo run build/dev-bundle --port 8143 --hidden --detach
```

When done, `rm -rf build/dev-bundle` (it's gitignored).

**Driving the app to recapture screenshots** — when a PNG got overwritten by the wrong screen or needs an updated capture:

```sh
OCTO_CARD_HOST=... tools/octo run <bundle> --port 8143 --hidden --detach
sleep 2
curl -s "127.0.0.1:8143/snap?q=<hint>" | python3 -c "
import json,sys
d=json.load(sys.stdin)
# walk d['s'] looking for {'t': '<hint>'}, print rect
"
# click center of returned rect
curl -s "127.0.0.1:8143/click?x=<cx>&y=<cy>&wait=1" >/dev/null
# fill focused text input
curl -s "127.0.0.1:8143/t?t=<text>&wait=1" >/dev/null
# trigger action
curl -s "127.0.0.1:8143/click?x=<bx>&y=<by>&wait=1" >/dev/null
sleep <gen-time>
# grab PNG
curl -s "127.0.0.1:8143/g?raw=1" -o promo/media/<file>.png
curl -s "127.0.0.1:8143/quit" >/dev/null
```

Snap returns 412×803 window coordinates; click coords are also CSS points.

**`hub publish` failing on `anchor did not certify this working key`** — almost always means `anchor.key` got overwritten by an accidental `hub keygen $KEYS/anchor.key`. Re-run the publish; the new working cert will be made under the new anchor. **Important**: this invalidates the previous catalog sequence. Wipe `build/mirror/catalog.json` and re-publish from sequence 1 if you don't care about history. Then update `build/anchor.hex` to the new pubkey (`hub pubkey $KEYS/anchor.key`).

**git 443 timeout but gh works** — `git push origin <branch>` will hang. Use `gh api`:

```sh
# create a single commit blob→tree→commit→ref, fast-forward main
# (only works for single-commit changes; multi-commit history needs git push)
gh api repos/<owner>/<repo>/git/ref/heads/main    # get base SHA
# build blob, then tree (base_tree=<base tree SHA>), then commit (parent=base SHA), then PATCH refs/heads/main
```

For tag pushes: lightweight `git tag <name> <SHA>` then `gh api repos/<o>/<r>/git/refs -X POST -f ref=refs/tags/<name> -f sha=<SHA>`. Annotated tags fail with `Object does not exist` because GitHub requires the tag-object SHA, not the commit SHA — use lightweight tags or push via git when network is up.

## References

Detailed recipes and worked examples:

- `references/scene-cookbook.md` — copy-paste scene skeletons for grid, phone, desktop-window, storyboard timeline, arc-fan, multi-platform
- `references/audio-mix.md` — full AudioMap recipes and LUFS debugging
- `references/drive-app.md` — full /snap /click /t /g automation with example payloads

Read them when you need a starting template; otherwise the SKILL.md above is the short list of rules that always apply.
