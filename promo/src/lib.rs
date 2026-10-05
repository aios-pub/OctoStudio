//! OctoStudio promo v9 — three hosts, real captures, full scenario coverage
use fframes::{
    AnimateRuntimeInput, AudioMap, AudioTrack, Color, Duration, FFramesContext, Frame, Scene,
    Scenes, Svgr, Transform, Video,
    animation::{AnimationRuntime, Easing},
    include_media_dir,
};
use fframes::AudioTimestamp::{Eof, Second};
use std::sync::LazyLock;

include_media_dir!(pub struct PromoMedia, "media");

pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;

const BG: &str = "#0a0c11";
const BG_CARD: &str = "#141821";
const INK: &str = "#eef1f6";
const INK_SOFT: &str = "#8891a3";
const ORANGE: &str = "#ff6b35";
const ORANGE_GLOW: &str = "#ff9466";
const ORANGE_DIM: &str = "#2c1d17";
const GRID: &str = "#161a23";
const PIPE: &str = "#252b38";
const FONT: &str = "Arial Unicode MS";
const WEIGHT: u16 = 500;

static EASE_OUT: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(0.9, &Easing::CubicBezier(0.16, 1.0, 0.3, 1.0))
});
static SPRING: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(4.0, &Easing::Spring { mass: 1.0, stiffness: 220.0, damping: 15.0 })
});
static DIP: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(0.32, &Easing::CubicBezier(0.3, 0.0, 0.2, 1.0))
});
static WIPE: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(0.55, &Easing::CubicBezier(0.7, 0.0, 0.3, 1.0))
});
static KB: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(6.0, &Easing::CubicBezier(0.33, 0.0, 0.4, 1.0))
});

fn ramp(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.0, to: 1.0, animation_runtime: &EASE_OUT })
}
fn rise(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 70.0, to: 0.0, animation_runtime: &SPRING })
}
fn slide_from_right(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 300.0, to: 0.0, animation_runtime: &EASE_OUT })
}
fn slide_from_left(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: -300.0, to: 0.0, animation_runtime: &EASE_OUT })
}
fn scale_pop(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.3, to: 1.0, animation_runtime: &SPRING })
}
fn kenburns(frame: &Frame) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: 0.0, from: 1.0, to: 1.045, animation_runtime: &KB })
}
fn dip_in(frame: &Frame) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: 0.0, from: 1.0, to: 0.0, animation_runtime: &DIP })
}
fn dip_out(frame: &Frame, scene_len: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: scene_len - 0.7, from: 0.0, to: 1.0, animation_runtime: &DIP })
}
fn wipe_parts(frame: &Frame) -> Vec<Svgr<'static>> {
    let p: f32 = frame.animate_runtime(AnimateRuntimeInput { on_second: 0.0, from: 0.0, to: 1.0, animation_runtime: &WIPE });
    let p = p.clamp(0.0, 1.0);
    if p >= 1.0 {
        return Vec::new();
    }
    let x = p * 1920.0;
    vec![
        fframes::svgr!(<rect x={x - 1920.0} y="0" width="1920" height="1080" fill={BG} />),
        fframes::svgr!(<rect x={x - 5.0} y="0" width="4" height="1080" fill={ORANGE_GLOW} opacity="0.65" />),
    ]
}

/// Flowing light dots along a straight pipe segment.
fn pipe_with_flow<'a>(x1: f32, y1: f32, x2: f32, y2: f32, t: f32, phase: f32) -> Vec<Svgr<'a>> {
    let mut out = Vec::new();
    out.push(fframes::svgr!(<line x1={x1} y1={y1} x2={x2} y2={y2} stroke={PIPE} stroke-width="6" stroke-linecap="round" />));
    out.push(fframes::svgr!(<line x1={x1} y1={y1} x2={x2} y2={y2} stroke={ORANGE} stroke-width="1.5" stroke-linecap="round" opacity="0.3" />));
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    let ux = dx / len;
    let uy = dy / len;
    for k in 0..3 {
        let p = (t * 0.35 + phase + k as f32 * 0.33) % 1.0;
        let px = x1 + dx * p;
        let py = y1 + dy * p;
        let trail = 0.12;
        let tx = px - ux * len * trail;
        let ty = py - uy * len * trail;
        out.push(fframes::svgr!(<line x1={tx} y1={ty} x2={px} y2={py} stroke={ORANGE_GLOW} stroke-width="3" stroke-linecap="round" opacity="0.7" />));
        out.push(fframes::svgr!(<circle cx={px} cy={py} r="4" fill={ORANGE} />));
    }
    out
}

/// Hexagon outline decoration.
fn hexagon<'a>(cx: f32, cy: f32, r: f32, opacity: f32) -> Svgr<'a> {
    let mut pts = String::new();
    for i in 0..6 {
        let a = i as f32 * 1.0472;
        pts.push_str(&format!("{:.1},{:.1} ", cx + a.cos() * r, cy + a.sin() * r));
    }
    fframes::svgr!(<polygon points={pts} fill="none" stroke={ORANGE} stroke-width="1" opacity={opacity} />)
}

// =========================================================================
// Video root
// =========================================================================

pub struct PromoVideo<'a> {
    pub media: &'a PromoMedia,
    intro: IntroScene,
    philosophy: PhilosophyScene,
    plaza: PlazaScene,
    grid: ScenarioGridScene,
    compose: ComposeScene,
    article: ArticleScene,
    video: VideoStoryScene,
    more: MoreFormatsScene,
    flow: FlowScene,
    constellation: ConstellationScene,
    export: ExportScene,
    hosts_intro: HostsIntroScene,
    hosts_desktop: HostsDesktopScene,
    hosts_card: HostsCardScene,
    hosts_rinx: HostsRinxScene,
    outro: OutroScene,
}

impl<'a> PromoVideo<'a> {
    pub fn new(media: &'a PromoMedia, _title: &'a str) -> Self {
        Self {
            media,
            intro: IntroScene,
            philosophy: PhilosophyScene,
            plaza: PlazaScene,
            grid: ScenarioGridScene,
            compose: ComposeScene,
            article: ArticleScene,
            video: VideoStoryScene,
            more: MoreFormatsScene,
            flow: FlowScene,
            constellation: ConstellationScene,
            export: ExportScene,
            hosts_intro: HostsIntroScene,
            hosts_desktop: HostsDesktopScene,
            hosts_card: HostsCardScene,
            hosts_rinx: HostsRinxScene,
            outro: OutroScene,
        }
    }
}

impl std::fmt::Debug for PromoVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("PromoVideo").finish() }
}

impl Video for PromoVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::hex(BG);

    fn duration(&self) -> Duration<'_> { Duration::Auto }

    /// BGM covers the whole 60s timeline; whooshes mark the big scene cuts,
    /// pops land on click/phone-entrance beats.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("bgm60.wav", Second(0.)..Eof).gain_db(-5.0).fade_in(1.5).fade_out(2.5),
            // whooshes at big transitions
            AudioTrack::new("whoosh-sfx.wav", Second(6.85)..Eof).volume(1.4),
            AudioTrack::new("whoosh-sfx.wav", Second(12.85)..Eof).volume(1.4),
            AudioTrack::new("whoosh-sfx.wav", Second(30.85)..Eof).volume(1.4),
            AudioTrack::new("whoosh-sfx.wav", Second(35.35)..Eof).volume(1.4),
            AudioTrack::new("whoosh-sfx.wav", Second(48.85)..Eof).volume(1.4),
            // hosts_intro entrance beat
            AudioTrack::new("whoosh-sfx.wav", Second(50.95)..Eof).volume(1.4),
            // pops: plaza click + phones
            AudioTrack::new("pop-sfx.wav", Second(9.3)..Eof).volume(1.4),
            AudioTrack::new("pop-sfx.wav", Second(31.35)..Eof).volume(1.0),
            AudioTrack::new("pop-sfx.wav", Second(31.8)..Eof).volume(1.0),
            AudioTrack::new("pop-sfx.wav", Second(44.8)..Eof).volume(0.7),
            AudioTrack::new("pop-sfx.wav", Second(45.4)..Eof).volume(0.7),
            AudioTrack::new("pop-sfx.wav", Second(46.0)..Eof).volume(0.7),
        ])
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![
            &self.intro as &dyn Scene,
            &self.philosophy,
            &self.plaza,
            &self.grid,
            &self.compose,
            &self.article,
            &self.video,
            &self.more,
            &self.flow,
            &self.constellation,
            &self.export,
            &self.hosts_intro,
            &self.hosts_desktop,
            &self.hosts_card,
            &self.hosts_rinx,
            &self.outro,
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let mut bg_parts: Vec<Svgr> = Vec::new();

        for i in 0..13 {
            let x = i as f32 * 160.0;
            bg_parts.push(fframes::svgr!(<line x1={x} y1="0" x2={x} y2="1080" stroke={GRID} stroke-width="0.5" />));
        }
        for i in 0..8 {
            let y = i as f32 * 150.0;
            bg_parts.push(fframes::svgr!(<line x1="0" y1={y} x2="1920" y2={y} stroke={GRID} stroke-width="0.5" />));
        }
        for i in 0..20 {
            let px = (i as f32 * 97.0 + t * (10.0 + i as f32)) % 1920.0;
            let py = (i as f32 * 73.0 + t * (7.0 + i as f32 * 0.8) + (i as f32 * 41.0).sin() * 60.0) % 1080.0;
            let po = 0.08 + (i as f32 * 11.0).sin().abs() * 0.15;
            bg_parts.push(fframes::svgr!(<circle cx={px} cy={py} r="1.5" fill={ORANGE} opacity={po} />));
        }
        bg_parts.push(fframes::svgr!(<circle cx={960.0 + (t * 10.0).sin() * 600.0} cy={540.0 + (t * 7.0).cos() * 350.0} r="700" fill={ORANGE} opacity="0.03" />));

        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={BG} />
                {bg_parts}
                {ctx.render_scenes(&frame)}
            </svg>
        )
    }
}

// =========================================================================
// Mockup components
// =========================================================================

fn phone_with_image<'a>(x: f32, y: f32, sc: f32, kb: f32, img_path: &'a str) -> Svgr<'a> {
    let kx = 170.0 * (1.0 - kb);
    let ky = 350.0 * (1.0 - kb);
    fframes::svgr!(<g transform={format!("translate({} {}) scale({})", x, y, sc)}>
        <rect x="-8" y="-8" width="356" height="716" rx="50" fill="#080808" />
        <rect x="0" y="0" width="340" height="700" rx="42" fill="#faf6f1" />
        <g transform={format!("translate({} {}) scale({})", kx, ky, kb)}>
            <image href={img_path} x="0" y="0" width="340" height="700" preserveAspectRatio="xMidYMid slice" />
        </g>
        <rect x="130" y="10" width="80" height="6" rx="3" fill="#333" opacity="0.6" />
        <rect x="-8" y="-8" width="356" height="716" rx="50" fill="none" stroke={ORANGE} stroke-width="1" opacity="0.3" />
    </g>)
}

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
        <rect x="-10" y="-10" width="1020" height="620" rx="18" fill="none" stroke={ORANGE} stroke-width="1" opacity="0.25" />
    </g>)
}

fn tech_tags<'a>(t: f32, tags: &'static [(&'static str, f32, f32)]) -> Vec<Svgr<'static>> {
    tags.iter().enumerate().map(|(i, (label, x, y))| {
        let bob = (t * 1.5 + i as f32 * 2.0).sin() * 8.0;
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} transform={format!("translate({} {})", x, y + bob)}>
            <rect x="-60" y="-18" width="120" height="36" rx="18" fill={BG_CARD} stroke={ORANGE_DIM} stroke-width="1" />
            <text x="0" y="5" text-anchor="middle" font-size="13" fill={INK_SOFT}>{*label}</text>
        </g>)
    }).collect()
}

// =========================================================================
// 1. Intro (3.0s)
// =========================================================================

#[derive(Debug)]
struct IntroScene;
impl Scene for IntroScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(3.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let logo_op = ramp(&frame, 0.2);
        let pulse = 0.5 + 0.5 * (t * 3.0).sin();
        let title_op = ramp(&frame, 0.8);
        let sub_op = ramp(&frame, 1.3);

        let mut particles: Vec<Svgr> = Vec::new();
        for i in 0..20 {
            let angle = i as f32 * 0.314;
            let dist = 300.0 - ramp(&frame, 0.15 + i as f32 * 0.03) * 280.0;
            let px = 960.0 + angle.cos() * dist;
            let py = 300.0 + angle.sin() * dist * 0.5;
            particles.push(fframes::svgr!(<circle cx={px} cy={py} r="3" fill={ORANGE} opacity={1.0 - ramp(&frame, 0.15 + i as f32 * 0.03) * 0.6} />));
        }

        let rot1 = t * 15.0;
        let rot2 = -t * 10.0;

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("rotate({} 960 300)", rot1)}>{hexagon(960.0, 300.0, 130.0, 0.25)}</g>
            <g transform={format!("rotate({} 960 300)", rot2)}>{hexagon(960.0, 300.0, 100.0, 0.15)}</g>
            {particles}
            <g opacity={logo_op} transform={Transform::translate(0.0, rise(&frame, 0.2))}>
                <rect x="890" y="230" width="140" height="140" rx="32" fill={ORANGE} />
                <rect x="890" y="230" width="140" height="140" rx="32" fill={ORANGE} opacity={0.25 + pulse * 0.15} />
                <text x="960" y="325" text-anchor="middle" font-size="58" font-weight="700" fill="#fff">{"OS"}</text>
            </g>
            <g opacity={title_op} transform={Transform::translate(0.0, rise(&frame, 0.8))}>
                <text x="960" y="470" text-anchor="middle" font-size="76" letter-spacing="-2" fill={INK}>{"OctoStudio"}</text>
            </g>
            <g opacity={sub_op}>
                <text x="960" y="530" text-anchor="middle" font-size="26" letter-spacing="6" fill={ORANGE}>{"言出法随 · 意图即创作"}</text>
            </g>
        </g>)
    }
}

// =========================================================================
// 2. Philosophy (3.0s)
// =========================================================================

#[derive(Debug)]
struct PhilosophyScene;
impl Scene for PhilosophyScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(3.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let items = [
            ("言出法随", "说出口的话，就是可发布的范式"),
            ("意图即创作", "不需要模板，不需要配置，意图即路径"),
        ];
        let item_svgs: Vec<Svgr> = items.iter().enumerate().map(|(i, (h, d))| {
            let st = 0.3 + i as f32 * 1.0;
            let op = ramp(&frame, st);
            let y = rise(&frame, st);
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={Transform::translate(0.0, y)}>
                <rect x="360" y={350.0 + i as f32 * 180.0} width="1200" height="130" rx="20" fill={BG_CARD} />
                <rect x="360" y={350.0 + i as f32 * 180.0} width="6" height="130" rx="3" fill={ORANGE} />
                <text x="410" y={410.0 + i as f32 * 180.0} font-size="40" font-weight="700" fill={INK}>{*h}</text>
                <text x="410" y={455.0 + i as f32 * 180.0} font-size="22" fill={INK_SOFT}>{*d}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="240" text-anchor="middle" font-size="22" letter-spacing="4" fill={ORANGE}>{"PRODUCT PHILOSOPHY"}</text>
            {item_svgs}
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 3. Plaza (5.0s)
// =========================================================================

#[derive(Debug)]
struct PlazaScene;
impl Scene for PlazaScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(5.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let slide = slide_from_right(&frame, 0.3);
        let labels_op = ramp(&frame, 1.0);

        let click_x = 939.0f32;
        let click_y = 523.0f32;
        let click_at = 2.0f32;

        let cursor_x = frame.animate_runtime(AnimateRuntimeInput {
            on_second: 0.9, from: 1300.0, to: click_x, animation_runtime: &EASE_OUT
        });
        let cursor_y = frame.animate_runtime(AnimateRuntimeInput {
            on_second: 0.9, from: 850.0, to: click_y, animation_runtime: &EASE_OUT
        });
        let cursor_op = if t < 0.9 { ramp(&frame, 0.75) } else if t < click_at + 0.35 { 1.0 } else { 1.0 - ramp(&frame, click_at + 0.35) };

        let since = (t - click_at).max(0.0);

        let mut ripple_parts: Vec<Svgr> = Vec::new();
        for k in 0..4 {
            let rp = ((since - k as f32 * 0.07) / 0.8).clamp(0.0, 1.0);
            if rp > 0.0 && rp < 1.0 {
                let rr = rp * 200.0;
                let ro = (1.0 - rp) * 0.75;
                ripple_parts.push(fframes::svgr!(<circle cx={click_x} cy={click_y} r={rr} fill="none" stroke={ORANGE_GLOW} stroke-width="2.5" opacity={ro} />));
            }
        }
        let burst = (1.0 - (since / 0.35).clamp(0.0, 1.0)).max(0.0);
        if burst > 0.0 {
            ripple_parts.push(fframes::svgr!(<circle cx={click_x} cy={click_y} r={10.0 + (1.0-burst)*40.0} fill={ORANGE} opacity={burst * 0.5} />));
        }

        let flash = if since > 0.0 {
            let ft = (since / 0.55).clamp(0.0, 1.0);
            if ft < 0.3 { ft / 0.3 * 0.85 } else { (1.0 - (ft - 0.3) / 0.7) * 0.85 }
        } else { 0.0 };

        let transition = ramp(&frame, 2.3);
        let win_sc = 1.0 - transition * 0.25;
        let win_op = (1.0 - transition).max(0.0);

        let target_op = ramp(&frame, 2.65);
        let target_sc = scale_pop(&frame, 2.65);
        let target_rise = rise(&frame, 2.65);

        static TAGS: &[(&str, f32, f32)] = &[
            ("原文二创", 200.0, 440.0),
            ("图文文章", 200.0, 530.0),
            ("视频分镜", 200.0, 620.0),
            ("PPT演示", 200.0, 710.0),
        ];
        let tag_svgs: Vec<Svgr> = TAGS.iter().enumerate().map(|(i, (label, x, y))| {
            let bob = (t * 1.5 + i as f32 * 2.0).sin() * 8.0;
            let op = labels_op * win_op;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={format!("translate({} {})", x, y + bob)}>
                <rect x="-60" y="-18" width="120" height="36" rx="18" fill={BG_CARD} stroke={ORANGE_DIM} stroke-width="1" />
                <text x="0" y="5" text-anchor="middle" font-size="13" fill={INK_SOFT}>{*label}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g opacity={win_op} transform={format!("translate({} 0) scale({}) translate({} {})", slide, win_sc, 720.0, 290.0)}>
                <g transform={format!("scale({})", 1.05)}>
                    {desktop_window(0.0, 0.0, 1.0, kenburns(&frame), "01-plaza.png")}
                </g>
            </g>
            <g opacity={labels_op * win_op}>
                <text x="200" y="300" font-size="48" font-weight="700" fill={INK}>{"创作广场"}</text>
                <text x="200" y="345" font-size="20" fill={INK_SOFT}>{"点击卡片，直达创作"}</text>
            </g>
            {tag_svgs}
            {ripple_parts}
            <g opacity={target_op} transform={format!("translate(0 {}) scale({})", target_rise, target_sc)}>
                {phone_with_image(790.0, 165.0, 1.08, kenburns(&frame), "06-video-comp.png")}
                <text x="960" y="960" text-anchor="middle" font-size="36" font-weight="700" fill={INK}>{"视频分镜 · 已打开"}</text>
            </g>
            <rect width="1920" height="1080" fill={ORANGE_GLOW} opacity={flash} />
            <g opacity={cursor_op} transform={format!("translate({} {})", cursor_x, cursor_y)}>
                <path d="M 0 0 L 0 20 L 5 14.5 L 8 21 L 10.5 20 L 7.5 13.5 L 13.5 13.5 Z" fill="#fff" stroke="#000" stroke-width="1.2" stroke-linejoin="round" />
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 4. ScenarioGrid (3.5s)
// =========================================================================

#[derive(Debug)]
struct ScenarioGridScene;
impl Scene for ScenarioGridScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(3.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        static SCENARIOS: &[(&str, &str)] = &[
            ("原文二创", "粘贴原文，改写/总结/润色"),
            ("图文文章", "一句话生成大纲与配图 prompt"),
            ("视频分镜", "一句话生成 4-6 镜分镜与合成包"),
            ("PPT 演示", "演示大纲一键成稿，导出 Marp"),
            ("拆解视频", "拆解爆款，提取可复用骨架"),
            ("标题工坊", "8 个候选标题，附推荐指数"),
            ("小红书笔记", "标题/正文/标签/配图一步到位"),
            ("口播稿", "钩子/节拍/CTA，真人出镜即用"),
            ("思维导图", "主题或原文变导图，导 markmap"),
            ("金句语录", "8 条金句，注明适用场景"),
        ];

        let card_w = 330.0f32;
        let card_h = 140.0f32;
        let gap_x = 33.0f32;
        let gap_y = 40.0f32;
        let x0 = (1920.0 - (card_w * 5.0 + gap_x * 4.0)) / 2.0;
        let y0 = 250.0f32;
        static BADGES: [&str; 10] = ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10"];

        let card_svgs: Vec<Svgr> = SCENARIOS.iter().enumerate().map(|(i, (name, hint))| {
            let col = (i % 5) as f32;
            let row = (i / 5) as f32;
            let x = x0 + col * (card_w + gap_x);
            let y = y0 + row * (card_h + gap_y);
            let st = 0.2 + i as f32 * 0.1;
            let op = ramp(&frame, st);
            let sc = scale_pop(&frame, st);
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={format!("translate({} {}) scale({})", x, y, sc)}>
                <rect width={card_w} height={card_h} rx="16" fill={BG_CARD} stroke={PIPE} stroke-width="1" />
                <rect x="0" y="0" width="56" height="4" rx="2" fill={ORANGE} />
                <text x="20" y="34" font-size="13" fill={ORANGE}>{BADGES[i]}</text>
                <text x="20" y="76" font-size="27" font-weight="700" fill={INK}>{*name}</text>
                <text x="20" y="112" font-size="13.5" fill={INK_SOFT}>{*hint}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="140" text-anchor="middle" font-size="44" font-weight="700" fill={INK}>{"10 个创作场景 · 一句话入场"}</text>
            <text x="960" y="185" text-anchor="middle" font-size="18" fill={INK_SOFT}>{"创作广场的每一张卡片，都是一条可发布的产线"}</text>
            {card_svgs}
            <text x="960" y="680" text-anchor="middle" font-size="16" fill={INK_SOFT} opacity={ramp(&frame, 1.5)}>{"全部产物进入通用计划编辑器，可改可重排 · 7 种格式导出"}</text>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 5. Compose (3.0s)
// =========================================================================

#[derive(Debug)]
struct ComposeScene;
impl Scene for ComposeScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(3.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let slide = slide_from_right(&frame, 0.3);

        static THEMES: &[(&str, &str)] = &[
            ("公众号深度", "#ff6b35"),
            ("干货清单",   "#e8b04b"),
            ("情感散文",   "#6ba6ff"),
            ("小红书种草", "#7fd0a8"),
            ("知乎科普",   "#c98bde"),
        ];
        let styles_svgs: Vec<Svgr> = THEMES.iter().enumerate().map(|(i, (name, accent))| {
            let st = 0.35 + i as f32 * 0.12;
            let op = ramp(&frame, st);
            let x = 200.0 + slide_from_left(&frame, st);
            let degrees = -3.0 + i as f32 * 1.5;
            let y = 480.0 + i as f32 * 62.0;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
                transform={format!("translate({} {}) rotate({})", x, y, degrees)}>
                <rect x="-8" y="-24" width="252" height="48" rx="24" fill="#1a2130" stroke={PIPE} stroke-width="1" />
                <rect x="-8" y="-24" width="6" height="48" rx="3" fill={accent} />
                <text x="18" y="6" font-size="17" fill={INK}>{*name}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("translate({} 0)", slide)} opacity={ramp(&frame, 0.3)}>
                {desktop_window(720.0, 290.0, 1.05, kenburns(&frame), "02-compose-theme.png")}
            </g>
            <text x="200" y="310" font-size="44" font-weight="700" fill={INK} opacity={ramp(&frame, 0.4)}>{"选择主题样式"}</text>
            <text x="200" y="358" font-size="22" fill={INK_SOFT} opacity={ramp(&frame, 0.55)}>{"10 种文章风格 · 一致语感"}</text>
            {styles_svgs}
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 6. Article (4.0s)
// =========================================================================

#[derive(Debug)]
struct ArticleScene;
impl Scene for ArticleScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let slide = slide_from_left(&frame, 0.3);
        let text_op = ramp(&frame, 1.0);
        let t = frame.seconds();

        static TAGS: &[(&str, f32, f32)] = &[
            ("AI 配图", 1150.0, 720.0),
            ("生图 prompt", 1370.0, 720.0),
            ("可编辑", 1150.0, 790.0),
        ];
        let tag_svgs = tech_tags(t, TAGS);

        let mut wave_pts = String::new();
        for i in 0..40 {
            let wx = 1080.0 + i as f32 * 9.0;
            let wy = 600.0 + (i as f32 * 0.5 + t * 2.0).sin() * 25.0;
            wave_pts.push_str(&format!("{:.1},{:.1} ", wx, wy));
        }

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("translate({} 0)", slide)} opacity={ramp(&frame, 0.3)}>
                {phone_with_image(300.0, 170.0, 1.05, kenburns(&frame), "03-article-images.png")}
            </g>
            <g opacity={text_op}>
                <text x="1080" y="380" font-size="44" font-weight="700" fill={INK}>{"图文文章"}</text>
                <text x="1080" y="430" font-size="22" fill={INK_SOFT}>{"标题 · 摘要 · 段落"}</text>
                <text x="1080" y="465" font-size="22" fill={INK_SOFT}>{"每段附 AI 配图"}</text>
            </g>
            <polyline points={wave_pts} fill="none" stroke={ORANGE} stroke-width="1.5" opacity="0.4" />
            {tag_svgs}
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 7. VideoStory (4.5s)
// =========================================================================

#[derive(Debug)]
struct VideoStoryScene;
impl Scene for VideoStoryScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let rise_y = rise(&frame, 0.3);
        let text_op = ramp(&frame, 1.0);
        let strip_op = ramp(&frame, 0.8);
        let t = frame.seconds();

        static TAGS: &[(&str, f32, f32)] = &[
            ("分镜表", 200.0, 380.0),
            ("合成建议", 200.0, 460.0),
            ("SRT字幕", 200.0, 540.0),
        ];
        let tag_svgs = tech_tags(t, TAGS);

        let cell_w = 130.0f32;
        let cell_h = 74.0f32;
        let pitch = 152.0f32;
        let strip_x = 200.0f32;
        let strip_y = 830.0f32;
        let sweep = ((t - 1.0) / 3.0).clamp(0.0, 1.0);
        let head_x = strip_x + sweep * 890.0;
        static SHOT_LABELS: [&str; 6] = ["镜1", "镜2", "镜3", "镜4", "镜5", "镜6"];
        let strip_svgs: Vec<Svgr> = (0..6usize).map(|i| {
            let cx = strip_x + i as f32 * pitch;
            let lit = head_x >= cx + cell_w * 0.5;
            let border = if lit { ORANGE } else { PIPE };
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
                <rect x={cx} y={strip_y} width={cell_w} height={cell_h} rx="8" fill={BG_CARD} stroke={border} stroke-width="1.5" />
                <text x={cx + cell_w / 2.0} y={strip_y + cell_h / 2.0 + 5.0} text-anchor="middle" font-size="14" fill={if lit { INK } else { INK_SOFT }}>{SHOT_LABELS[i]}</text>
                <circle cx={cx + cell_w / 2.0} cy={strip_y + cell_h + 16.0} r="4" fill={if lit { ORANGE } else { PIPE }} />
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={Transform::translate(0.0, rise_y)} opacity={ramp(&frame, 0.3)}>
                {phone_with_image(880.0, 105.0, 0.98, kenburns(&frame), "06-video-comp.png")}
            </g>
            <g opacity={text_op}>
                <text x="200" y="230" font-size="44" font-weight="700" fill={INK}>{"视频分镜"}</text>
                <text x="200" y="275" font-size="20" fill={INK_SOFT}>{"转场 · 配乐 · 调色"}</text>
            </g>
            {tag_svgs}
            <g opacity={strip_op}>
                <text x="200" y="808" font-size="15" fill={INK_SOFT}>{"分镜时间轴 · 逐镜可编辑"}</text>
                <line x1={strip_x} y1={strip_y + cell_h + 16.0} x2={strip_x + 890.0} y2={strip_y + cell_h + 16.0} stroke={PIPE} stroke-width="2" />
                {strip_svgs}
                {if sweep > 0.0 && sweep < 1.0 {
                    fframes::svgr!(<g>
                        <line x1={head_x} y1={strip_y - 8.0} x2={head_x} y2={strip_y + cell_h + 8.0} stroke={ORANGE_GLOW} stroke-width="2" opacity="0.85" />
                        <circle cx={head_x} cy={strip_y - 8.0} r="4" fill={ORANGE_GLOW} />
                    </g>)
                } else { fframes::svgr!(<g />) }}
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 8. MoreFormats (3.5s)
// =========================================================================

#[derive(Debug)]
struct MoreFormatsScene;
impl Scene for MoreFormatsScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(3.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let cap_a = ramp(&frame, 0.55);
        let cap_b = ramp(&frame, 0.95);
        let text_op = ramp(&frame, 0.4);

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g opacity={text_op}>
                <text x="200" y="360" font-size="44" font-weight="700" fill={INK}>{"不止图文和分镜"}</text>
                <text x="200" y="412" font-size="22" fill={INK_SOFT}>{"同一套通用计划编辑器"}</text>
                <text x="200" y="448" font-size="22" fill={INK_SOFT}>{"每条可编辑 · 上移/下移/删除 · 存为模板"}</text>
            </g>
            <g opacity={ramp(&frame, 0.3)} transform={Transform::translate(0.0, rise(&frame, 0.3))}>
                {phone_with_image(760.0, 170.0, 0.92, kenburns(&frame), "05-ppt-plan.png")}
            </g>
            <g opacity={ramp(&frame, 0.65)} transform={Transform::translate(0.0, rise(&frame, 0.65))}>
                {phone_with_image(1190.0, 170.0, 0.92, kenburns(&frame), "10-teardown.png")}
            </g>
            <text x="928" y="935" text-anchor="middle" font-size="18" font-weight="700" fill={INK} opacity={cap_a}>{"PPT 演示 · 导出 Marp 即成演示"}</text>
            <text x="1360" y="935" text-anchor="middle" font-size="18" font-weight="700" fill={INK} opacity={cap_b}>{"拆解视频 · 骨架存为分镜模板"}</text>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 9. Flow (4.0s)
// =========================================================================

#[derive(Debug)]
struct FlowScene;
impl Scene for FlowScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();

        let nodes = [
            (220.0, 350.0, "输入意图", "一句话/一段原文"),
            (620.0, 350.0, "Splash VM", "沙箱解释执行"),
            (1020.0, 550.0, "model.complete", "按 schema 生成"),
            (1420.0, 550.0, "结构化 Plan", "可编辑产物"),
            (1700.0, 350.0, "多格式导出", "7 种格式"),
        ];

        let mut pipe_parts: Vec<Svgr> = Vec::new();
        for i in 0..nodes.len() - 1 {
            let (x1, y1, ..) = nodes[i];
            let (x2, y2, ..) = nodes[i + 1];
            let pipe_active = ramp(&frame, 0.4 + i as f32 * 0.5);
            let mut seg = pipe_with_flow(x1 + 55.0, y1, x2 - 55.0, y2, t, i as f32 * 0.2);
            for s in seg.drain(..) {
                pipe_parts.push(fframes::svgr!(<g opacity={pipe_active}>{s}</g>));
            }
        }

        let node_svgs: Vec<Svgr> = nodes.iter().enumerate().map(|(i, (x, y, title, desc))| {
            let activate = ramp(&frame, 0.25 + i as f32 * 0.5);
            let sc = scale_pop(&frame, 0.25 + i as f32 * 0.5);
            let glow = 0.3 + activate * 0.4 + (t * 3.0 + i as f32).sin().abs() * 0.1;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} transform={format!("translate({} {}) scale({})", x, y, sc)}>
                <rect x="-55" y="-45" width="110" height="90" rx="14" fill={BG_CARD} stroke={ORANGE} stroke-width="1.5" />
                <rect x="-55" y="-45" width="110" height="90" rx="14" fill={ORANGE} opacity={glow * 0.15} />
                <text x="0" y="-5" text-anchor="middle" font-size="16" font-weight="700" fill={INK}>{*title}</text>
                <text x="0" y="20" text-anchor="middle" font-size="11" fill={INK_SOFT}>{*desc}</text>
                <circle cx="0" cy="-45" r="4" fill={ORANGE} opacity={activate} />
            </g>)
        }).collect();

        let wipe = wipe_parts(&frame);
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="180" text-anchor="middle" font-size="44" font-weight="700" fill={INK}>{"执行流 · 意图到产物"}</text>
            <text x="960" y="225" text-anchor="middle" font-size="20" fill={INK_SOFT}>{"数据在管道中流动，每一步都有明确的契约"}</text>
            {pipe_parts}
            {node_svgs}
            {wipe}
        </g>)
    }
}

// =========================================================================
// 10. Constellation (4.0s)
// =========================================================================

#[derive(Debug)]
struct ConstellationScene;
impl Scene for ConstellationScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();

        let stars = [
            ("Splash VM", 300.0, 350.0, 22.0, 0),
            ("model.complete", 700.0, 250.0, 26.0, 1),
            ("octos.turn", 550.0, 600.0, 20.0, 2),
            ("images", 1050.0, 450.0, 18.0, 3),
            ("storage", 400.0, 800.0, 18.0, 4),
            ("Markdown", 1350.0, 300.0, 20.0, 5),
            ("公众号", 1550.0, 550.0, 18.0, 6),
            ("Notion", 1200.0, 750.0, 18.0, 7),
            ("Marp", 1600.0, 800.0, 16.0, 8),
            ("SRT", 900.0, 850.0, 16.0, 9),
            ("ffmpeg", 1700.0, 300.0, 16.0, 10),
            ("markmap", 850.0, 150.0, 16.0, 11),
        ];

        let links: [(usize, usize); 16] = [
            (0,1),(0,2),(0,4),(1,3),(1,11),(2,3),(2,4),(3,5),
            (3,6),(3,9),(4,9),(5,6),(5,10),(6,7),(7,8),(7,9),
        ];

        let mut link_parts: Vec<Svgr> = Vec::new();
        for (a, b) in links.iter() {
            let (_, x1, y1, _, o1) = stars[*a];
            let (_, x2, y2, _, o2) = stars[*b];
            let appear = ramp(&frame, 0.2 + (o1.min(o2)) as f32 * 0.14);
            link_parts.push(fframes::svgr!(<g opacity={appear * 0.4}>
                <line x1={x1} y1={y1} x2={x2} y2={y2} stroke={ORANGE} stroke-width="1" />
            </g>));
            let flow_p = (t * 0.25 + (*a) as f32 * 0.17) % 1.0;
            let px = x1 + (x2 - x1) * flow_p;
            let py = y1 + (y2 - y1) * flow_p;
            link_parts.push(fframes::svgr!(<g opacity={appear}>
                <circle cx={px} cy={py} r="2.5" fill={ORANGE_GLOW} />
            </g>));
        }

        let star_svgs: Vec<Svgr> = stars.iter().map(|(label, x, y, r, order)| {
            let appear = ramp(&frame, 0.2 + *order as f32 * 0.14);
            let pulse = 0.6 + 0.4 * (t * 2.0 + *order as f32).sin();
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={appear}>
                <circle cx={*x} cy={*y} r={r + 12.0} fill={ORANGE} opacity={0.08 * pulse} />
                <circle cx={*x} cy={*y} r={*r} fill={BG_CARD} stroke={ORANGE} stroke-width="2" />
                <circle cx={*x} cy={*y} r={*r * 0.4} fill={ORANGE} opacity={pulse * 0.8} />
                <text x={*x} y={*y + *r + 20.0} text-anchor="middle" font-size="14" fill={INK}>{*label}</text>
            </g>)
        }).collect();

        let wipe = wipe_parts(&frame);
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="80" text-anchor="middle" font-size="44" font-weight="700" fill={INK}>{"技术栈星图"}</text>
            <text x="960" y="120" text-anchor="middle" font-size="18" fill={INK_SOFT}>{"平台能力 · 生成引擎 · 导出格式 — 每个节点都是契约"}</text>
            {link_parts}
            {star_svgs}
            {wipe}
        </g>)
    }
}

// =========================================================================
// 11. Export (3.5s)
// =========================================================================

#[derive(Debug)]
struct ExportScene;
impl Scene for ExportScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let formats = [
            ("07-export-srt.png", "SRT 字幕"),
            ("08-export-pack.png", "制作包"),
            ("09-export-marp.png", "Marp"),
        ];
        let phone_svgs: Vec<Svgr> = formats.iter().enumerate().map(|(i, (img, title))| {
            let st = 0.25 + i as f32 * 0.5;
            let op = ramp(&frame, st);
            let sc = scale_pop(&frame, st);
            let x = 400.0 + i as f32 * 400.0;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={format!("translate({} 180) scale({})", x, sc * 0.72)}>
                {phone_with_image(0.0, 0.0, 1.0, kenburns(&frame), img)}
                <text x="170" y="750" text-anchor="middle" font-size="20" font-weight="700" fill={INK}>{*title}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="110" text-anchor="middle" font-size="44" font-weight="700" fill={INK}>{"多格式导出"}</text>
            <text x="960" y="150" text-anchor="middle" font-size="18" fill={INK_SOFT}>{"Markdown · 公众号 · Notion · Marp · SRT · 制作包 · markmap"}</text>
            {phone_svgs}
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 12. HostsIntro (2.0s) — quick three-host triptych
// =========================================================================

#[derive(Debug)]
struct HostsIntroScene;
impl Scene for HostsIntroScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(1.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.15);
        // Three glass cards arranged in a fan, each labeled with a host name
        let labels = ["card-host", "OctoSense 桌面壳", "Rinx 小程序"];
        let cards: Vec<Svgr> = (0..3usize).map(|i| {
            let st = 0.25 + i as f32 * 0.12;
            let op = ramp(&frame, st);
            let sc = scale_pop(&frame, st);
            let deg = (-12.0 + i as f32 * 12.0);
            let cx = 960.0 + (i as f32 - 1.0) * 320.0;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
                transform={format!("translate({} 580) rotate({}) scale({})", cx, deg, sc)}>
                <rect x="-160" y="-200" width="320" height="380" rx="18" fill="#10141d" stroke={ORANGE} stroke-width="1.5" opacity="0.85" />
                <rect x="-160" y="-200" width="320" height="6" rx="3" fill={ORANGE} />
                <text x="0" y="-30" text-anchor="middle" font-size="20" font-weight="700" fill={INK}>{labels[i]}</text>
                <rect x="-130" y="0" width="260" height="160" rx="10" fill="#0a0c11" />
                <text x="0" y="220" text-anchor="middle" font-size="14" fill={INK_SOFT}>{"真实运行 · 真实截图"}</text>
            </g>)
        }).collect();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="160" text-anchor="middle" font-size="48" font-weight="700" fill={INK} opacity={title_op}>{"三种宿主,一处运行"}</text>
            <text x="960" y="210" text-anchor="middle" font-size="20" fill={INK_SOFT} opacity={title_op}>{"App Card · 一次发布,处处启动"}</text>
            {cards}
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 13. HostsDesktop (4.5s) — OctoSense shell + host icon shown as a tile
// =========================================================================

#[derive(Debug)]
struct HostsDesktopScene;
impl Scene for HostsDesktopScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let shell_op = ramp(&frame, 0.4);
        let shell_sc = scale_pop(&frame, 0.4);
        let cap_op   = ramp(&frame, 1.5);

        // Shell mockup — scaled-down screenshot framed as a window
        let win_w = 960.0_f32;
        let win_h = 540.0_f32;
        let cx = 960.0_f32;
        let cy = 540.0_f32;

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK} opacity={title_op}>{"OctoSense 桌面壳"}</text>
            <text x="960" y="160" text-anchor="middle" font-size="19" fill={INK_SOFT} opacity={title_op}>{"本地优先 · 设备原生 AI · 多应用并行"}</text>

            <g opacity={shell_op} transform={format!("translate({} {}) scale({})", cx, cy, shell_sc)}>
                <g transform={format!("translate({} {})", -win_w/2.0, -win_h/2.0)}>
                    <rect x="-10" y="-10" width={win_w + 20.0} height={win_h + 20.0} rx="20" fill="#080808" />
                    <rect x="0" y="0" width={win_w} height={win_h} rx="12" fill="#0a0c11" />
                    <image href="h-shell.png" x="0" y="0" width={win_w} height={win_h} preserveAspectRatio="xMidYMid slice" />
                    <circle cx="14" cy="-18" r="5" fill="#ff5f57" />
                    <circle cx="32" cy="-18" r="5" fill="#febc2e" />
                    <circle cx="50" cy="-18" r="5" fill="#28c840" />
                    <rect x="-10" y="-10" width={win_w + 20.0} height={win_h + 20.0} rx="20" fill="none" stroke={ORANGE} stroke-width="1" opacity="0.3" />
                </g>
            </g>

            <text x="960" y="912" text-anchor="middle" font-size="16" fill={INK_SOFT} opacity={cap_op}>{"作品存设备沙箱 · 跨设备不离开你的控制"}</text>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 14. HostsCard (4.0s) — card-host mobile / tablet / desktop
// =========================================================================

#[derive(Debug)]
struct HostsCardScene;
impl Scene for HostsCardScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let mob_op = ramp(&frame, 0.35);
        let tab_op = ramp(&frame, 0.55);
        let dsk_op = ramp(&frame, 0.75);
        let mob_sc = scale_pop(&frame, 0.35);
        let tab_sc = scale_pop(&frame, 0.55);
        let dsk_sc = scale_pop(&frame, 0.75);

        // Phone (left): 824x1606 source → display ~280x546
        let mob_w = 280.0; let mob_h = 546.0;
        let mob_x = 220.0; let mob_y = 240.0;
        // Tablet (center): 2048x1536 source → display ~480x360
        let tab_w = 480.0; let tab_h = 360.0;
        let tab_x = 720.0; let tab_y = 380.0;
        // Desktop (right): 2560x1600 source → display ~640x400
        let dsk_w = 640.0; let dsk_h = 400.0;
        let dsk_x = 1240.0; let dsk_y = 360.0;

        let phone_kx = (mob_w / 2.0) * (1.0 - 1.0);
        let phone_ky = (mob_h / 2.0) * (1.0 - 1.0);

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK} opacity={title_op}>{"card-host · CLI 工具"}</text>
            <text x="960" y="160" text-anchor="middle" font-size="19" fill={INK_SOFT} opacity={title_op}>{"同一份 bundle · 三个窗口尺寸"}</text>

            // Phone
            <g opacity={mob_op} transform={format!("translate({} {}) scale({})", mob_x + mob_w/2.0, mob_y + mob_h/2.0, mob_sc)}>
                <g transform={format!("translate({} {})", -mob_w/2.0, -mob_h/2.0)}>
                    <rect x="-8" y="-8" width={mob_w + 16.0} height={mob_h + 16.0} rx="34" fill="#080808" />
                    <rect x="0" y="0" width={mob_w} height={mob_h} rx="28" fill="#faf6f1" />
                    <g transform={format!("translate({} {})", phone_kx, phone_ky)}>
                        <image href="h-mobile.png" x="0" y="0" width={mob_w} height={mob_h} preserveAspectRatio="xMidYMid slice" />
                    </g>
                    <rect x={(mob_w - 70.0) / 2.0} y="6" width="70" height="5" rx="2.5" fill="#333" opacity="0.6" />
                    <rect x="-8" y="-8" width={mob_w + 16.0} height={mob_h + 16.0} rx="34" fill="none" stroke={ORANGE} stroke-width="1" opacity="0.3" />
                </g>
            </g>
            <text x={mob_x + mob_w/2.0} y={mob_y + mob_h + 50.0} text-anchor="middle" font-size="16" fill={INK_SOFT} opacity={mob_op}>{"手机 412×803"}</text>

            // Tablet
            <g opacity={tab_op} transform={format!("translate({} {}) scale({})", tab_x + tab_w/2.0, tab_y + tab_h/2.0, tab_sc)}>
                <g transform={format!("translate({} {})", -tab_w/2.0, -tab_h/2.0)}>
                    <rect x="-10" y="-10" width={tab_w + 20.0} height={tab_h + 20.0} rx="14" fill="#080808" />
                    <rect x="0" y="0" width={tab_w} height={tab_h} rx="6" fill="#faf6f1" />
                    <image href="h-tablet.png" x="0" y="0" width={tab_w} height={tab_h} preserveAspectRatio="xMidYMid slice" />
                    <circle cx="14" cy="-18" r="5" fill="#ff5f57" />
                    <circle cx="32" cy="-18" r="5" fill="#febc2e" />
                    <circle cx="50" cy="-18" r="5" fill="#28c840" />
                </g>
            </g>
            <text x={tab_x + tab_w/2.0} y={tab_y + tab_h + 50.0} text-anchor="middle" font-size="16" fill={INK_SOFT} opacity={tab_op}>{"平板 1024×768"}</text>

            // Desktop
            <g opacity={dsk_op} transform={format!("translate({} {}) scale({})", dsk_x + dsk_w/2.0, dsk_y + dsk_h/2.0, dsk_sc)}>
                <g transform={format!("translate({} {})", -dsk_w/2.0, -dsk_h/2.0)}>
                    <rect x="-10" y="-10" width={dsk_w + 20.0} height={dsk_h + 20.0} rx="14" fill="#080808" />
                    <rect x="0" y="0" width={dsk_w} height={dsk_h} rx="6" fill="#faf6f1" />
                    <image href="h-desktop.png" x="0" y="0" width={dsk_w} height={dsk_h} preserveAspectRatio="xMidYMid slice" />
                    <circle cx="14" cy="-18" r="5" fill="#ff5f57" />
                    <circle cx="32" cy="-18" r="5" fill="#febc2e" />
                    <circle cx="50" cy="-18" r="5" fill="#28c840" />
                </g>
            </g>
            <text x={dsk_x + dsk_w/2.0} y={dsk_y + dsk_h + 50.0} text-anchor="middle" font-size="16" fill={INK_SOFT} opacity={dsk_op}>{"桌面 1280×800"}</text>

            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 15. HostsRinx (4.5s) — Rinx Mini apps running OctoStudio
// =========================================================================

#[derive(Debug)]
struct HostsRinxScene;
impl Scene for HostsRinxScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let import_op = ramp(&frame, 0.4);
        let run_op    = ramp(&frame, 0.7);
        let ass_op    = ramp(&frame, 1.0);
        let import_sc = scale_pop(&frame, 0.4);
        let run_sc    = scale_pop(&frame, 0.7);
        let ass_sc    = scale_pop(&frame, 1.0);

        // Layout: three browser windows stacked vertically at different scales
        // top-left: Mini apps import page; top-right: OctoStudio running inside
        // bottom: Ask Rinx assistant panel
        let im_w = 720.0; let im_h = 360.0;
        let im_x = 100.0;  let im_y = 230.0;
        let ru_w = 720.0; let ru_h = 360.0;
        let ru_x = 1100.0; let ru_y = 230.0;
        let as_w = 980.0; let as_h = 280.0;
        let as_x = 470.0;  let as_y = 690.0;

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK} opacity={title_op}>{"Rinx 小程序"}</text>
            <text x="960" y="160" text-anchor="middle" font-size="19" fill={INK_SOFT} opacity={title_op}>{"Mini apps · Import 审查 → Run → Ask Rinx 助手"}</text>

            // Import window
            <g opacity={import_op} transform={format!("translate({} {}) scale({})", im_x + im_w/2.0, im_y + im_h/2.0, import_sc)}>
                <g transform={format!("translate({} {})", -im_w/2.0, -im_h/2.0)}>
                    <rect x="-10" y="-10" width={im_w + 20.0} height={im_h + 20.0} rx="14" fill="#080808" />
                    <rect x="0" y="0" width={im_w} height={im_h} rx="6" fill="#faf6f1" />
                    <image href="h-rinx-import.png" x="0" y="0" width={im_w} height={im_h} preserveAspectRatio="xMidYMid slice" />
                    <circle cx="14" cy="-18" r="5" fill="#ff5f57" />
                    <circle cx="32" cy="-18" r="5" fill="#febc2e" />
                    <circle cx="50" cy="-18" r="5" fill="#28c840" />
                </g>
            </g>
            <text x={im_x + im_w/2.0} y={im_y + im_h + 38.0} text-anchor="middle" font-size="15" fill={INK_SOFT} opacity={import_op}>{"① Import · 审查 bundle 能力清单"}</text>

            // Run window
            <g opacity={run_op} transform={format!("translate({} {}) scale({})", ru_x + ru_w/2.0, ru_y + ru_h/2.0, run_sc)}>
                <g transform={format!("translate({} {})", -ru_w/2.0, -ru_h/2.0)}>
                    <rect x="-10" y="-10" width={ru_w + 20.0} height={ru_h + 20.0} rx="14" fill="#080808" />
                    <rect x="0" y="0" width={ru_w} height={ru_h} rx="6" fill="#faf6f1" />
                    <image href="h-rinx-run.png" x="0" y="0" width={ru_w} height={ru_h} preserveAspectRatio="xMidYMid slice" />
                    <circle cx="14" cy="-18" r="5" fill="#ff5f57" />
                    <circle cx="32" cy="-18" r="5" fill="#febc2e" />
                    <circle cx="50" cy="-18" r="5" fill="#28c840" />
                </g>
            </g>
            <text x={ru_x + ru_w/2.0} y={ru_y + ru_h + 38.0} text-anchor="middle" font-size="15" fill={INK_SOFT} opacity={run_op}>{"② Run · OctoStudio 在 Mini apps 内运行"}</text>

            // Ask Rinx panel
            <g opacity={ass_op} transform={format!("translate({} {}) scale({})", as_x + as_w/2.0, as_y + as_h/2.0, ass_sc)}>
                <g transform={format!("translate({} {})", -as_w/2.0, -as_h/2.0)}>
                    <rect x="-10" y="-10" width={as_w + 20.0} height={as_h + 20.0} rx="14" fill="#080808" />
                    <rect x="0" y="0" width={as_w} height={as_h} rx="6" fill="#faf6f1" />
                    <image href="h-rinx-assistant.png" x="0" y="0" width={as_w} height={as_h} preserveAspectRatio="xMidYMid slice" />
                </g>
            </g>
            <text x={as_x + as_w/2.0} y={as_y + as_h + 38.0} text-anchor="middle" font-size="15" fill={INK_SOFT} opacity={ass_op}>{"③ Ask Rinx · 壳内助手可调起 OctoStudio"}</text>

            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 16. Outro (5.0s)
// =========================================================================

#[derive(Debug)]
struct OutroScene;
impl Scene for OutroScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(4.5) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let logo_op = ramp(&frame, 0.2);
        let title_op = ramp(&frame, 0.45);
        let slogan_op = ramp(&frame, 0.75);
        let info_op = ramp(&frame, 3.7);
        let fade = dip_out(&frame, 5.0);

        let rot = t * 12.0;

        let chars: &[(char, bool)] = &[
            ('愿', false), ('每', false), ('一', false), ('个', false),
            ('意', true), ('图', true), ('，', false),
            ('落', false), ('地', false), ('即', false),
            ('产', true), ('物', true),
        ];
        let char_base = 1.0f32;
        let char_gap = 0.1f32;
        let fs = 42f32;
        let pitch = 48f32;
        let total_w = (chars.len() - 1) as f32 * pitch;
        let start_x = 960.0 - total_w / 2.0;
        let line_y = 620.0f32;

        let char_svgs: Vec<Svgr> = chars.iter().enumerate().map(|(i, (ch, highlight))| {
            let st = char_base + i as f32 * char_gap;
            let op = ramp(&frame, st);
            let sc = frame.animate_runtime(AnimateRuntimeInput {
                on_second: st, from: 1.8f32, to: 1.0f32, animation_runtime: &SPRING
            });
            let cx = start_x + i as f32 * pitch;
            let color = if *highlight { ORANGE } else { INK };
            let spark = if t >= st {
                1.0 - ((t - st) / 0.6).clamp(0.0, 1.0)
            } else { 0.0 };
            let breathe = 1.0 + (t * 2.0 + i as f32).sin() * 0.02;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} transform={format!("translate({} {})", cx, line_y)}>
                {if *highlight {
                    let halo = 0.3 + (t * 2.5 + i as f32).sin().abs() * 0.25;
                    fframes::svgr!(<g>
                        <circle cx="0" cy="-8" r="38" fill={ORANGE} opacity={halo * 0.4 * op} />
                        <circle cx="0" cy="-8" r="28" fill={ORANGE} opacity={halo * op} />
                    </g>)
                } else { fframes::svgr!(<g />) }}
                <g transform={format!("scale({} {})", sc * breathe, sc * breathe)} opacity={op}>
                    <text x="0" y="0" text-anchor="middle" font-size={fs} font-weight="700" fill={color}>{ch.to_string()}</text>
                </g>
                {if spark > 0.0 {
                    let mut sp: Vec<Svgr> = Vec::new();
                    for k in 0..6 {
                        let a = k as f32 * 1.047 + i as f32 * 0.5;
                        let d = (1.0 - spark) * 38.0;
                        let sx = a.cos() * d;
                        let sy = -8.0 + a.sin() * d;
                        sp.push(fframes::svgr!(<line x1={sx - a.cos()*6.0} y1={sy - a.sin()*6.0} x2={sx} y2={sy} stroke={ORANGE_GLOW} stroke-width="2" stroke-linecap="round" opacity={spark*0.9} />));
                        sp.push(fframes::svgr!(<circle cx={sx} cy={sy} r="2.5" fill="#fff" opacity={spark} />));
                    }
                    fframes::svgr!(<g>{sp}</g>)
                } else { fframes::svgr!(<g />) }}
            </g>)
        }).collect();

        let sweep_progress = ramp(&frame, 2.3);
        let sweep_cx = start_x - 40.0 + sweep_progress * (total_w + 80.0);
        let sweep_op = (0.22 * (1.0 - ((sweep_progress - 0.5) * 2.0).powi(2))).max(0.0);

        let mut up_particles: Vec<Svgr> = Vec::new();
        let particle_t0 = 2.0f32;
        if t >= particle_t0 {
            let lt = t - particle_t0;
            for i in 0..14 {
                let pt = (lt * 0.28 + i as f32 * 0.15) % 1.0;
                let px = start_x + 20.0 + (i as f32 * 127.0 % (total_w - 20.0));
                let py = line_y + 30.0 - pt * 150.0;
                let po = (1.0 - pt) * 0.8;
                up_particles.push(fframes::svgr!(<g>
                    <circle cx={px} cy={py} r="5" fill={ORANGE} opacity={po * 0.25} />
                    <circle cx={px} cy={py} r="2.5" fill={ORANGE_GLOW} opacity={po} />
                </g>));
            }
        }

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("rotate({} 960 250)", rot)} opacity={logo_op}>
                {hexagon(960.0, 250.0, 110.0, 0.2)}
            </g>
            <g opacity={logo_op} transform={Transform::translate(0.0, rise(&frame, 0.2))}>
                <rect x="895" y="185" width="130" height="130" rx="30" fill={ORANGE} />
                <text x="960" y="273" text-anchor="middle" font-size="54" font-weight="700" fill="#fff">{"OS"}</text>
            </g>
            <g opacity={title_op} transform={Transform::translate(0.0, rise(&frame, 0.45))}>
                <text x="960" y="390" text-anchor="middle" font-size="60" letter-spacing="-1" fill={INK}>{"OctoStudio"}</text>
            </g>
            <g opacity={slogan_op}>
                <text x="960" y="455" text-anchor="middle" font-size="26" letter-spacing="6" fill={ORANGE}>{"言出法随 · 意图即创作"}</text>
            </g>
            {char_svgs}
            <ellipse cx={sweep_cx} cy={line_y - 18.0} rx="70" ry="42" fill="#fff" opacity={sweep_op * 0.5} />
            <ellipse cx={sweep_cx} cy={line_y - 18.0} rx="30" ry="20" fill="#fff" opacity={sweep_op} />
            {up_particles}
            <rect x={start_x - 30.0} y={line_y + 22.0} width={total_w + 60.0} height="3" fill={ORANGE} opacity={0.55 * ramp(&frame, 2.1)} />
            <rect x={start_x - 30.0} y={line_y + 22.0} width={total_w + 60.0} height="8" fill={ORANGE} opacity={0.18 * ramp(&frame, 2.1)} />
            <g opacity={info_op}>
                <text x="960" y="730" text-anchor="middle" font-size="15" fill={INK_SOFT}>{"Rinx · OctoSense App Hub · Apache-2.0 · v0.3.3"}</text>
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={fade} />
        </g>)
    }
}
