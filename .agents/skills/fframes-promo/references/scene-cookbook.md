# Scene cookbook

All snippets assume `FONT`, `BG_CARD`, `ORANGE`, `PIPE`, `INK`, `INK_SOFT`, `GRID`, `EASE_OUT`, `SPRING` and helpers (`ramp`, `rise`, `slide_from_right`, `scale_pop`, `kenburns`) exist in lib.rs. Static label arrays are necessary because of the svgr borrow trap — see SKILL.md pitfall #1.

## 1. Title card with hex rings + particles

```rust
#[derive(Debug)]
struct TitleScene;
impl Scene for TitleScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let logo_op = ramp(&frame, 0.25);
        let title_op = ramp(&frame, 0.9);
        let pulse = 0.5 + 0.5 * (t * 3.0).sin();

        let mut particles: Vec<Svgr> = Vec::new();
        for i in 0..20 {
            let angle = i as f32 * 0.314;
            let dist = 300.0 - ramp(&frame, 0.15 + i as f32 * 0.03) * 280.0;
            let px = 960.0 + angle.cos() * dist;
            let py = 300.0 + angle.sin() * dist * 0.5;
            particles.push(fframes::svgr!(<circle cx={px} cy={py} r="3" fill={ORANGE}
                opacity={1.0 - ramp(&frame, 0.15 + i as f32 * 0.03) * 0.6} />));
        }

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("rotate({} 960 300)", t * 15.0)}>{hexagon(960.0, 300.0, 130.0, 0.25)}</g>
            <g transform={format!("rotate({} 960 300)", -t * 10.0)}>{hexagon(960.0, 300.0, 100.0, 0.15)}</g>
            {particles}
            <g opacity={logo_op} transform={Transform::translate(0.0, rise(&frame, 0.25))}>
                <rect x="890" y="230" width="140" height="140" rx="32" fill={ORANGE} />
                <rect x="890" y="230" width="140" height="140" rx="32" fill={ORANGE} opacity={0.25 + pulse * 0.15} />
                <text x="960" y="325" text-anchor="middle" font-size="58" font-weight="700" fill="#fff">{"OS"}</text>
            </g>
            <g opacity={title_op} transform={Transform::translate(0.0, rise(&frame, 0.9))}>
                <text x="960" y="470" text-anchor="middle" font-size="76" letter-spacing="-2" fill={INK}>{"Title"}</text>
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={1.0 - ramp(&frame, 0.0)} />
        </g>)
    }
}
```

## 2. Scenario card grid (2×5)

```rust
static SCENARIOS: &[(&str, &str)] = &[
    ("原文二创", "粘贴原文,改写/润色"),
    ("图文文章", "一句话生成图文大纲"),
    // ...
];
static BADGES: [&str; 10] = ["01","02","03","04","05","06","07","08","09","10"];

let card_w = 330.0; let card_h = 140.0; let gap_x = 33.0; let gap_y = 40.0;
let x0 = (1920.0 - (card_w * 5.0 + gap_x * 4.0)) / 2.0;
let y0 = 250.0;
let card_svgs: Vec<Svgr> = SCENARIOS.iter().enumerate().map(|(i, (name, hint))| {
    let col = (i % 5) as f32;
    let row = (i / 5) as f32;
    let x = x0 + col * (card_w + gap_x);
    let y = y0 + row * (card_h + gap_y);
    let st = 0.25 + i as f32 * 0.12;
    fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}
        opacity={ramp(&frame, st)}
        transform={format!("translate({} {}) scale({})", x, y, scale_pop(&frame, st))}>
        <rect width={card_w} height={card_h} rx="16" fill={BG_CARD} stroke={PIPE} stroke-width="1" />
        <rect x="0" y="0" width="56" height="4" rx="2" fill={ORANGE} />
        <text x="20" y="34" font-size="13" fill={ORANGE}>{BADGES[i]}</text>
        <text x="20" y="76" font-size="27" font-weight="700" fill={INK}>{*name}</text>
        <text x="20" y="112" font-size="13.5" fill={INK_SOFT}>{*hint}</text>
    </g>)
}).collect();
```

## 3. Phone mockup with Ken Burns

```rust
fn phone_with_image<'a>(x: f32, y: f32, sc: f32, kb: f32, img_path: &'a str) -> Svgr<'a> {
    let kx = 170.0 * (1.0 - kb);   // half image width * (1 - scale)
    let ky = 350.0 * (1.0 - kb);
    fframes::svgr!(<g transform={format!("translate({} {}) scale({})", x, y, sc)}>
        <rect x="-8" y="-8" width="356" height="716" rx="50" fill="#080808" />
        <rect x="0" y="0" width="340" height="700" rx="42" fill="#faf6f1" />
        <g transform={format!("translate({} {}) scale({})", kx, ky, kb)}>
            <image href={img_path} x="0" y="0" width="340" height="700" preserveAspectRatio="xMidYMid slice" />
        </g>
        <rect x="130" y="10" width="80" height="6" rx="3" fill="#333" opacity="0.6" />
    </g>)
}
```

Call with `phone_with_image(x, y, sc, kenburns(&frame), "media-file.png")`. The scale `1.0→1.045` over 6 seconds gives a barely-perceptible push.

## 4. Desktop window with traffic-light dots

```rust
fn desktop_window<'a>(x: f32, y: f32, sc: f32, kb: f32, img_path: &'a str) -> Svgr<'a> {
    let kx = 500.0 * (1.0 - kb);
    let ky = 300.0 * (1.0 - kb);
    fframes::svgr!(<g transform={format!("translate({} {}) scale({})", x, y, sc)}>
        <rect x="-10" y="-10" width="1020" height="620" rx="18" fill="#080808" />
        <rect x="0" y="0" width="1000" height="600" rx="10" fill="#faf6f1" />
        <g transform={format!("translate({} {}) scale({})", kx, ky, kb)}>
            <image href={img_path} x="0" y="0" width="1000" height="600" preserveAspectRatio="xMidYMid slice" />
        </g>
        <circle cx="20" cy="-22" r="6" fill="#ff5f57" />
        <circle cx="40" cy="-22" r="6" fill="#febc2e" />
        <circle cx="60" cy="-22" r="6" fill="#28c840" />
    </g>)
}
```

## 5. Storyboard timeline with sweeping playhead

```rust
let cell_w = 130.0; let cell_h = 74.0; let pitch = 152.0;
let strip_x = 200.0; let strip_y = 830.0;
let sweep = ((t - 1.0) / 3.0).clamp(0.0_f32, 1.0);
let head_x = strip_x + sweep * 890.0;
static SHOT_LABELS: [&str; 6] = ["镜1","镜2","镜3","镜4","镜5","镜6"];
let cells: Vec<Svgr> = (0..6usize).map(|i| {
    let cx = strip_x + i as f32 * pitch;
    let lit = head_x >= cx + cell_w * 0.5;
    let border = if lit { ORANGE } else { PIPE };
    fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
        <rect x={cx} y={strip_y} width={cell_w} height={cell_h} rx="8"
              fill={BG_CARD} stroke={border} stroke-width="1.5" />
        <text x={cx + cell_w / 2.0} y={strip_y + cell_h / 2.0 + 5.0}
              text-anchor="middle" font-size="14"
              fill={if lit { INK } else { INK_SOFT }}>{SHOT_LABELS[i]}</text>
        <circle cx={cx + cell_w / 2.0} cy={strip_y + cell_h + 16.0} r="4"
                fill={if lit { ORANGE } else { PIPE }} />
    </g>)
}).collect();
```

## 6. Per-character text reveal (logo outro)

```rust
let chars: &[(char, bool)] = &[
    ('愿', false), ('每', false), ('一', false), ('个', false),
    ('意', true),  ('图', true),  ('，', false),
];
let char_base = 1.2_f32; let char_gap = 0.11; let fs = 46.0; let pitch = 50.0;
let total_w = (chars.len() - 1) as f32 * pitch;
let start_x = 960.0 - total_w / 2.0;
let line_y = 620.0;
let char_svgs: Vec<Svgr> = chars.iter().enumerate().map(|(i, (ch, highlight))| {
    let st = char_base + i as f32 * char_gap;
    let op = ramp(&frame, st);
    let sc = frame.animate_runtime(AnimateRuntimeInput {
        on_second: st, from: 1.8_f32, to: 1.0, animation_runtime: &SPRING
    });
    let cx = start_x + i as f32 * pitch;
    let color = if *highlight { ORANGE } else { INK };
    fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}
        transform={format!("translate({} {})", cx, line_y)}>
        <g transform={format!("scale({} {})", sc, sc)} opacity={op}>
            <text x="0" y="0" text-anchor="middle" font-size={fs} font-weight="700" fill={color}>
                {ch.to_string()}
            </text>
        </g>
    </g>)
}).collect();
```

## 7. Three-form showcase (phone, desktop, Rinx-style)

```rust
let phone_op  = ramp(&frame, 0.35); let phone_up  = rise(&frame, 0.35);
let desk_op   = ramp(&frame, 0.60); let desk_up   = rise(&frame, 0.60);
let third_op  = ramp(&frame, 0.85); let third_up  = rise(&frame, 0.85);

fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
    <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK}>
        {"App Card · 一次创作,处处运行"}
    </text>

    <g opacity={phone_op} transform={Transform::translate(0.0, phone_up)}>
        {phone_with_image(95.0, 240.0, 0.66, 1.0, "phone-capture.png")}
        <text x="222" y="748" text-anchor="middle" font-size="17" fill={INK}>
            {"手机端 · 竖屏单列"}
        </text>
    </g>
    <g opacity={desk_op} transform={Transform::translate(0.0, desk_up)}>
        {desktop_window(630.0, 300.0, 0.62, 1.0, "desktop-capture.png")}
        <text x="940" y="748" text-anchor="middle" font-size="17" fill={INK}>
            {"桌面端 · 桌面壳"}
        </text>
    </g>
    <g opacity={third_op} transform={Transform::translate(0.0, third_up)}>
        {desktop_window(1295.0, 300.0, 0.62, 1.0, "third-capture.png")}
        <text x="1605" y="748" text-anchor="middle" font-size="17" fill={INK}>
            {"Rinx · Mini apps"}
        </text>
    </g>

    <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
</g>)
```

**Note on positioning** — the three windows have centers x=222, x=940, x=1605 with widths ~250 each. Window widths are scaled 0.66 (phone) and 0.62 (desktop, 1000×600 base). Tune until your text labels don't crash into the windows.

## 8. Horizontal pill list (replaces dangerous arc/fan layouts)

When you want to show a list of similar items with light variation, do **not** rotate cards around a common pivot — labels collide. Instead, stack horizontal pills with small per-item rotation:

```rust
static THEMES: &[(&str, &str)] = &[
    ("公众号深度", "#ff6b35"),
    ("干货清单",   "#e8b04b"),
    ("情感散文",   "#6ba6ff"),
];
let styles_svgs: Vec<Svgr> = THEMES.iter().enumerate().map(|(i, (name, accent))| {
    let st = 0.45 + i as f32 * 0.14;
    let x = 200.0 + slide_from_left(&frame, st);
    let degrees = -3.0 + i as f32 * 1.5;
    let y = 480.0 + i as f32 * 62.0;
    fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}
        opacity={ramp(&frame, st)}
        transform={format!("translate({} {}) rotate({})", x, y, degrees)}>
        <rect x="-8" y="-24" width="252" height="48" rx="24" fill="#1a2130" stroke={PIPE} stroke-width="1" />
        <rect x="-8" y="-24" width="6" height="48" rx="3" fill={accent} />
        <text x="18" y="6" font-size="17" fill={INK}>{*name}</text>
    </g>)
}).collect();
```

Tiny rotation (-3° to +3°) gives organic feel without label collision.
