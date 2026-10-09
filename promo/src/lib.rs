//! OctoStudio promo v10 — v0.4.3 全品类:10 主题 / 5 视频预设 / M3 内容管理 / AI 助手 7 项
//! 18 景精确 60.000s @ 30fps;v9(16 景/60s)+ M3Content + AiAssistant + 主题×10 + 视频预设×5
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

// =========================================================================
// Subtitles (TTS Chinese narration, v12 sync-fixed) — 21 cues timed to
// actual TTS speech (not the padded target window). Audio map delays the
// whole narration.wav by 0.4s, so all cue starts shift +0.4s. Each cue
// lingers 0.3s after speech ends for natural reading tail.
// Total: 102.94s of speech out of 168s of padded narration track.
// =========================================================================
const SUBTITLES: &[(f32, f32, &str)] = &[
    // Act 1 — 痛 (0–20s; TTS speech 3.70/5.52/5.74)
    ( 0.40,  4.40, "你打开十三个标签页,只是为了发一篇文章。"),
    ( 7.40, 13.22, "复制,粘贴,调格式,再粘一次。换平台,再调一遍。"),
    (13.40, 19.44, "中文内容创作者的痛,是跨平台、跨格式、跨工具的复制循环。"),
    // Act 2 — 觉醒 (20–50s; TTS speech 3.86/5.59/4.42/4.56)
    (20.40, 24.56, "OctoStudio。本地 AI 创作工作台。"),
    (28.40, 34.29, "言出法随,意图即创作。说出口的话,就是可发布的范式。"),
    (37.40, 42.12, "你的设备、你的作品、你的模型,数据不离开沙箱。"),
    (43.40, 48.26, "一句话意图,七种出口。无需账号,无需密钥。"),
    // Act 3 — 演示 (50–110s; TTS speech 3.65/3.31/5.04/5.50/4.66/5.93)
    (50.40, 54.35, "九大场景一键触达,不必从空白开始。"),
    (60.40, 64.01, "写一句话意图,代替三十分钟构思。"),
    (70.40, 75.74, "AI 按 schema 出 plan,标题、摘要、段落,样样齐全。"),
    (78.40, 84.20, "视频分镜:四到六镜,带时长、景别、配音、生视频提示。"),
    (88.40, 93.36, "通用计划编辑器,改一改,就是你的稿,不是 AI 的稿。"),
    (96.40,102.63, "一份内容、七个出口,公众号、Notion、Marp、SRT,一句不动。"),
    // Act 4 — 能力 (110–145s; TTS speech 8.42/6.62/6.38/4.68)
    (110.40,119.12, "十种创作场景,覆盖图文、视频、PPT、拆解、标题、小红书、口播、导图、金句。"),
    (120.40,127.32, "搜索、标签、AI 历史、批量管理,你的作品库井井有条,跨设备不离手。"),
    (130.40,137.08, "AI 助手七项:起标题、打分、摘要、风格迁移,工坊屏里随调随用。"),
    (140.40,145.38, "十款主题、五种视频预设,风格注入随作品保存。"),
    // Act 5 — 实证 + 收尾 (cues 18–22 re-aligned to scene starts: 156/161/167/173/179)
    (156.40,160.78, "三种宿主,真实运行,不是 demo,是真活。"),
    (161.40,165.40, "OctoSense 桌面壳,本机模型真实生成。"),
    (167.40,170.94, "card-host 演示模式,作品展示完整。"),
    (173.40,178.04, "Rinx 小程序,原文二创与场景生成,部分降级。"),
    // NEW (v12.1) — PlatformsFutureScene at 179–186s (TTS speech 4.78s)
    (179.40,184.48, "未来,桌面、移动、Web,同一份 bundle 处处可跑。"),
];

/// Bottom-bar subtitle overlay (landscape 1920x1080). Drawn last so it
/// sits above every scene without per-scene code. Returns empty SVG
/// outside any cue window.
fn subtitle_overlay(global_t: f32) -> Svgr<'static> {
    let Some(&(start, end, text)) = SUBTITLES
        .iter()
        .find(|(s, e, _)| global_t >= *s && global_t <= *e)
    else {
        return Svgr::empty();
    };
    // 0.18s linear fade-in / fade-out — keeps transitions silent, no popping.
    let alpha = ((global_t - start) / 0.18).clamp(0.0, 1.0)
        .min(((end - global_t) / 0.18).clamp(0.0, 1.0));

    let bar_w = 1340.0_f32;
    let bar_h = 84.0_f32;
    let bar_x = (WIDTH as f32 - bar_w) / 2.0;
    let bar_y = HEIGHT as f32 * 0.86;

    fframes::svgr!(
        <g opacity={alpha} font-family={FONT} font-weight={WEIGHT}>
            <rect x={bar_x} y={bar_y} width={bar_w} height={bar_h} rx="14"
                  fill="#000" opacity="0.62" />
            <rect x={bar_x} y={bar_y} width="4" height={bar_h} rx="2"
                  fill={ORANGE} opacity="0.85" />
            <text x={WIDTH as f32 / 2.0} y={bar_y + bar_h * 0.66}
                  text-anchor="middle" font-size="28" font-weight="500"
                  fill="#fff" letter-spacing="1">{text}</text>
        </g>
    )
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
    // Act 1 — 痛
    pain_chaos: PainChaosScene,
    pain_clipboard: PainClipboardScene,
    pain_voice: PainVoiceScene,
    // Act 2 — 觉醒
    intro: IntroScene,
    philosophy: PhilosophyScene,
    // Act 3 — 演示
    plaza: PlazaScene,
    compose: ComposeScene,
    article: ArticleScene,
    video: VideoStoryScene,
    edit: EditPanelScene,
    more: MoreFormatsScene,
    // Act 4 — 能力
    grid: ScenarioGridScene,
    flow: FlowScene,
    constellation: ConstellationScene,
    export: ExportScene,
    m3_content: M3ContentScene,
    ai_assistant: AiAssistantScene,
    // Act 5 — 实证 + 收尾
    hosts_intro: HostsIntroScene,
    hosts_desktop: HostsDesktopScene,
    hosts_card: HostsCardScene,
    hosts_rinx: HostsRinxScene,
    platforms_future: PlatformsFutureScene,
    outro: OutroScene,
}

impl<'a> PromoVideo<'a> {
    pub fn new(media: &'a PromoMedia, _title: &'a str) -> Self {
        Self {
            media,
            // Act 1
            pain_chaos: PainChaosScene,
            pain_clipboard: PainClipboardScene,
            pain_voice: PainVoiceScene,
            // Act 2
            intro: IntroScene,
            philosophy: PhilosophyScene,
            // Act 3
            plaza: PlazaScene,
            compose: ComposeScene,
            article: ArticleScene,
            video: VideoStoryScene,
            edit: EditPanelScene,
            more: MoreFormatsScene,
            // Act 4
            grid: ScenarioGridScene,
            flow: FlowScene,
            constellation: ConstellationScene,
            export: ExportScene,
            m3_content: M3ContentScene,
            ai_assistant: AiAssistantScene,
            // Act 5
            hosts_intro: HostsIntroScene,
            hosts_desktop: HostsDesktopScene,
            hosts_card: HostsCardScene,
            hosts_rinx: HostsRinxScene,
            platforms_future: PlatformsFutureScene,
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

    /// BGM covers the whole 187s timeline; whooshes mark the big scene cuts,
    /// pops land on click/phone-entrance beats.
    /// v12: layered zh-CN-YunyangNeural voice-over (164s of speech over 187s video).
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("bgm199.wav", Second(0.)..Eof).gain_db(-5.0).fade_in(1.5).fade_out(2.5),
            // v12 voice-over — 0.4s delay; gain 8dB; .voice() flags as primary speech
            AudioTrack::new("narration.wav", Second(0.4)..Eof)
                .gain_db(8.0)
                .fade_in(0.05).fade_out(0.5)
                .voice(),
            // Act 1 → Act 2 觉醒
            AudioTrack::new("whoosh-sfx.wav", Second(7.0)..Eof).volume(1.0),   // PainChaos → PainClipboard
            AudioTrack::new("whoosh-sfx.wav", Second(13.0)..Eof).volume(1.0),  // PainClipboard → PainVoice
            AudioTrack::new("whoosh-sfx.wav", Second(20.0)..Eof).volume(1.5),  // Act 1 → Act 2 觉醒
            // Act 2 → Act 3
            AudioTrack::new("whoosh-sfx.wav", Second(50.0)..Eof).volume(1.4),  // → Plaza
            // Act 3 — 演示 (mid-acts)
            AudioTrack::new("whoosh-sfx.wav", Second(60.0)..Eof).volume(1.0),  // Plaza → Compose
            AudioTrack::new("whoosh-sfx.wav", Second(70.0)..Eof).volume(1.0),  // Compose → Article
            AudioTrack::new("whoosh-sfx.wav", Second(78.0)..Eof).volume(1.0),  // Article → Video
            AudioTrack::new("whoosh-sfx.wav", Second(88.0)..Eof).volume(1.0),  // Video → Edit
            AudioTrack::new("whoosh-sfx.wav", Second(96.0)..Eof).volume(1.2),  // Edit → Export
            // Act 3 → Act 4
            AudioTrack::new("whoosh-sfx.wav", Second(110.0)..Eof).volume(1.5), // → Grid
            // Act 4 内部
            AudioTrack::new("whoosh-sfx.wav", Second(140.0)..Eof).volume(1.0), // AI → Constellation
            // Act 4 → Act 5
            AudioTrack::new("whoosh-sfx.wav", Second(145.0)..Eof).volume(1.4), // → HostsIntro
            // Act 5 内部
            AudioTrack::new("whoosh-sfx.wav", Second(150.0)..Eof).volume(1.0), // → HostsDesktop
            AudioTrack::new("whoosh-sfx.wav", Second(156.0)..Eof).volume(1.0), // → HostsCard
            AudioTrack::new("whoosh-sfx.wav", Second(162.0)..Eof).volume(1.0), // → HostsRinx
            // NEW (v12.1) — PlatformsFutureScene at 173s
            AudioTrack::new("whoosh-sfx.wav", Second(173.0)..Eof).volume(1.4), // Rinx → PlatformsFuture
            // pops: Plaza click + UI beats
            AudioTrack::new("pop-sfx.wav", Second(5.5)..Eof).volume(1.4),     // Plaza click (was 9.3)
            AudioTrack::new("pop-sfx.wav", Second(31.5)..Eof).volume(1.0),    // constellation beat — unused now
            AudioTrack::new("pop-sfx.wav", Second(31.9)..Eof).volume(1.0),    // constellation beat 2
            AudioTrack::new("pop-sfx.wav", Second(75.5)..Eof).volume(0.7),    // Edit panel pop
            AudioTrack::new("pop-sfx.wav", Second(76.0)..Eof).volume(0.7),
            AudioTrack::new("pop-sfx.wav", Second(76.5)..Eof).volume(0.7),
        ])
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![
            // Act 1 — 痛
            &self.pain_chaos as &dyn Scene,
            &self.pain_clipboard,
            &self.pain_voice,
            // Act 2 — 觉醒
            &self.intro,
            &self.philosophy,
            // Act 3 — 演示
            &self.plaza,
            &self.compose,
            &self.article,
            &self.video,
            &self.edit,
            &self.more,
            // Act 4 — 能力
            &self.grid,
            &self.flow,
            &self.constellation,
            &self.export,
            &self.m3_content,
            &self.ai_assistant,
            // Act 5 — 实证 + 收尾
            &self.hosts_intro,
            &self.hosts_desktop,
            &self.hosts_card,
            &self.hosts_rinx,
            &self.platforms_future,
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
                {subtitle_overlay(t)}
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
// ACT 1 — 痛 (Pain, 20s, 3 scenes)
// =========================================================================

// 1. PainChaos — 7.0s — 13 tabs floating, "13" big counter ticking up
#[derive(Debug)]
struct PainChaosScene;
impl Scene for PainChaosScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(7.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let intro_op = ramp(&frame, 0.3);

        // 13 floating tab icons (browser/editor/notion/cloud/clipboard etc.) — each
        // has its own phase, drifts in/out. Counter ticks from 0 → 13 over the scene.
        let tabs: &[(&str, &str)] = &[
            ("◇", "编辑器"), ("▤", "Notion"), ("◯", "浏览器"),
            ("◫", "剪贴板"), ("▥", "网盘"), ("◊", "Markdown"),
            ("▦", "公众号"), ("✉", "邮件"), ("◬", "图片库"),
            ("▧", "PDF"), ("◐", "翻译"), ("◭", "便签"),
            ("▩", "收藏夹"),
        ];
        let tab_svgs: Vec<Svgr> = tabs.iter().enumerate().map(|(i, (icon, label))| {
            // Place on an outer ring (radius ~ 540), at angles 360/13 apart
            let angle = i as f32 * std::f32::consts::TAU / tabs.len() as f32 - std::f32::consts::FRAC_PI_2;
            let base_x = 960.0 + angle.cos() * 540.0;
            let base_y = 540.0 + angle.sin() * 320.0;
            let drift = (t * 0.6 + i as f32 * 0.7).sin() * 16.0;
            let cx = base_x + drift;
            let cy = base_y + (t * 0.4 + i as f32 * 0.5).cos() * 10.0;
            let enter = ramp(&frame, 0.4 + i as f32 * 0.12);
            let exit = 1.0 - ramp(&frame, 5.5 + i as f32 * 0.08);
            let op = enter * exit;
            fframes::svgr!(<g opacity={op} font-family={FONT} font-weight={WEIGHT}>
                <rect x={cx - 70.0} y={cy - 22.0} width="140" height="44" rx="6"
                      fill={BG_CARD} stroke={ORANGE_DIM} stroke-width="1" />
                <circle cx={cx - 50.0} cy={cy} r="6" fill={ORANGE} opacity="0.85" />
                <text x={cx - 38.0} y={cy + 5.0} font-size="14" fill={INK_SOFT}>{*icon}</text>
                <text x={cx - 22.0} y={cy + 5.0} font-size="15" fill={INK_SOFT}>{*label}</text>
            </g>)
        }).collect();

        // Counter: ticks from 0 → 13 over the scene, then overshoots to 13+
        let count_raw = ((t * 2.5).min(7.5) / 7.5 * 14.0).min(14.0) as i32;
        let count = count_raw.min(13);

        // "13 tabs" big headline — appears mid-scene
        let head_op = ramp(&frame, 3.0);
        let head_scale = scale_pop(&frame, 3.0);

        // Central "you are here" void with red dotted border — chaotic center
        let void_pulse = 0.5 + 0.5 * (t * 4.0).sin();

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            {tab_svgs}
            <g opacity={intro_op}>
                <circle cx="960" cy="540" r="180" fill="none"
                        stroke="#ff4444" stroke-width="1.5" stroke-dasharray="6,8"
                        opacity={0.4 + void_pulse * 0.3} />
                <circle cx="960" cy="540" r="140" fill="none"
                        stroke="#ff4444" stroke-width="1" stroke-dasharray="4,6"
                        opacity={0.3 + void_pulse * 0.2} />
                <text x="960" y="535" text-anchor="middle" font-size="120"
                      font-weight="700" fill="#ff6b6b">{count.to_string()}</text>
                <text x="960" y="595" text-anchor="middle" font-size="22"
                      fill={INK_SOFT} letter-spacing="6">{"TABS OPEN"}</text>
            </g>
            <g opacity={head_op} transform={format!("translate(960 870) scale({})", head_scale)}>
                <text x="0" y="0" text-anchor="middle" font-size="38"
                      font-weight="700" fill={INK}>{"你打开十三个标签页"}</text>
                <text x="0" y="40" text-anchor="middle" font-size="22" fill={INK_SOFT}>
                    {"只是为了发一篇文章。"}
                </text>
            </g>
        </g>)
    }
}

// 2. PainClipboard — 6.0s — copy/paste cycle with looping arrows
#[derive(Debug)]
struct PainClipboardScene;
impl Scene for PainClipboardScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let head_op = ramp(&frame, 0.2);
        // Two pulsing clipboard blocks at left/right with arrows looping between
        let left_op = 0.7 + 0.3 * (t * 2.0).sin();
        let right_op = 0.7 + 0.3 * (t * 2.0 + 1.5).sin();

        // Animated arrow progress (0→1→0 loop) for the "copy" arrow
        let copy_p = ((t * 1.2) % 1.0);
        let paste_p = ((t * 1.2 + 0.5) % 1.0);

        // Four clipboard cards representing different platforms
        let platforms: &[(&str, f32, f32, &str)] = &[
            ("编辑器", 360.0, 360.0, "#3a3a3a"),
            ("公众号", 1560.0, 360.0, "#3a3a3a"),
            ("Notion", 360.0, 720.0, "#3a3a3a"),
            ("小红书", 1560.0, 720.0, "#3a3a3a"),
        ];
        let card_svgs: Vec<Svgr> = platforms.iter().map(|(name, x, y, _)| {
            fframes::svgr!(<g opacity={head_op} font-family={FONT} font-weight={WEIGHT}>
                <rect x={x - 130.0} y={y - 80.0} width="260" height="160" rx="12"
                      fill={BG_CARD} stroke={ORANGE_DIM} stroke-width="1.5" />
                <rect x={x - 130.0} y={y - 80.0} width="260" height="22" rx="12"
                      fill={ORANGE} opacity="0.85" />
                <circle cx={x - 110.0} cy={y - 69.0} r="4" fill="#fff" />
                <circle cx={x - 95.0} cy={y - 69.0} r="4" fill="#fff" opacity="0.7" />
                <circle cx={x - 80.0} cy={y - 69.0} r="4" fill="#fff" opacity="0.5" />
                <text x={x} y={y - 18.0} text-anchor="middle" font-size="22"
                      font-weight="700" fill={INK}>{*name}</text>
                <text x={x} y={y + 10.0} text-anchor="middle" font-size="12"
                      fill={INK_SOFT}>{"格式 / 排版 / 重粘"}</text>
                <text x={x} y={y + 35.0} text-anchor="middle" font-size="12"
                      fill={INK_SOFT} opacity="0.7">{"又一遍"}</text>
                <text x={x} y={y + 60.0} text-anchor="middle" font-size="12"
                      fill={INK_SOFT} opacity="0.5">{"..."}</text>
            </g>)
        }).collect();

        // Arrows: top-left → top-right, bottom-left → bottom-right
        let mut arrows: Vec<Svgr> = Vec::new();
        // Top arrow (编辑器 → 公众号)
        let ax1 = 490.0 + copy_p * 1070.0;
        let ay1 = 360.0;
        arrows.push(fframes::svgr!(<g opacity={left_op.min(right_op) * (1.0 - copy_p * 0.3)}>
            <circle cx={ax1} cy={ay1} r="8" fill={ORANGE_GLOW} />
            <text x={ax1} y={ay1 + 5.0} text-anchor="middle" font-size="12"
                  fill="#000" font-weight="700">{">"}</text>
        </g>));
        // Bottom arrow (Notion → 小红书)
        let ax2 = 490.0 + paste_p * 1070.0;
        let ay2 = 720.0;
        arrows.push(fframes::svgr!(<g opacity={left_op.min(right_op) * (1.0 - paste_p * 0.3)}>
            <circle cx={ax2} cy={ay2} r="8" fill={ORANGE_GLOW} />
            <text x={ax2} y={ay2 + 5.0} text-anchor="middle" font-size="12"
                  fill="#000" font-weight="700">{">"}</text>
        </g>));
        // Return arrow (公众号 → 编辑器, looping back) — only visible in second half
        let return_p = ((t * 0.6 + 0.3) % 1.0);
        if t > 2.5 {
            let ax3 = 1560.0 - return_p * 1070.0;
            let ay3 = 540.0;
            arrows.push(fframes::svgr!(<g opacity={(t - 2.5) * 0.5 * (1.0 - return_p * 0.5)}>
                <circle cx={ax3} cy={ay3} r="6" fill="#ff6b6b" />
                <text x={ax3} y={ay3 + 4.0} text-anchor="middle" font-size="10"
                      fill="#000" font-weight="700">{"<"}</text>
            </g>));
        }

        // Headline at bottom
        let title_op = ramp(&frame, 1.5);
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            {card_svgs}
            {arrows}
            <g opacity={title_op}>
                <text x="960" y="940" text-anchor="middle" font-size="34"
                      font-weight="700" fill={INK}>{"复制,粘贴,调格式,再粘一次。"}</text>
                <text x="960" y="985" text-anchor="middle" font-size="22" fill={INK_SOFT}>
                    {"换平台,再调一遍。"}
                </text>
            </g>
        </g>)
    }
}

// 3. PainVoice — 7.0s — "中文内容创作者" reveal + four pain points
#[derive(Debug)]
struct PainVoiceScene;
impl Scene for PainVoiceScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(7.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        // 7 Chinese characters reveal one by one over 0.4..3.2s
        let chars = ['中', '文', '内', '容', '创', '作', '者'];
        let char_svgs: Vec<Svgr> = chars.iter().enumerate().map(|(i, c)| {
            let st = 0.3 + i as f32 * 0.32;
            let op = ramp(&frame, st);
            let rise_y = rise(&frame, st);
            let cx = 480.0 + i as f32 * 100.0;
            fframes::svgr!(<g opacity={op} font-family={FONT} font-weight={WEIGHT}
                            transform={format!("translate(0 {})", rise_y)}>
                <text x={cx} y="280" text-anchor="middle" font-size="72"
                      font-weight="700" fill={INK}>{c.to_string()}</text>
            </g>)
        }).collect();

        // Four pain points appear in a 2×2 grid below, one per second starting 3.5s
        let pains = [
            ("多平台分发繁琐", "同一篇稿粘四次"),
            ("AI 改写生硬", "跟语气对不上"),
            ("视频脚本难起手", "缺一座桥"),
            ("工具散在各处", "参考资料靠脑子"),
        ];
        let pain_svgs: Vec<Svgr> = pains.iter().enumerate().map(|(i, (title, sub))| {
            let st = 3.6 + i as f32 * 0.6;
            let op = ramp(&frame, st);
            let col = i % 2;
            let row = i / 2;
            let x = 540.0 + col as f32 * 480.0;
            let y = 480.0 + row as f32 * 150.0;
            fframes::svgr!(<g opacity={op} font-family={FONT} font-weight={WEIGHT}>
                <rect x={x - 200.0} y={y - 50.0} width="400" height="110" rx="14"
                      fill={BG_CARD} stroke="#ff6b6b" stroke-width="1.2" opacity="0.85" />
                <rect x={x - 200.0} y={y - 50.0} width="6" height="110" rx="3"
                      fill="#ff6b6b" />
                <text x={x - 175.0} y={y - 18.0} font-size="22" font-weight="700" fill={INK}>{*title}</text>
                <text x={x - 175.0} y={y + 15.0} font-size="15" fill={INK_SOFT}>{*sub}</text>
            </g>)
        }).collect();

        // Final frame headline summarizing the four
        let closing_op = ramp(&frame, 6.0);
        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            {char_svgs}
            {pain_svgs}
            <g opacity={closing_op}>
                <text x="960" y="950" text-anchor="middle" font-size="22"
                      fill={ORANGE} letter-spacing="4">{"一个跨平台、跨格式、跨工具的复制循环"}</text>
            </g>
        </g>)
    }
}

// =========================================================================
// 4. Intro (8.0s) — was 3.0s; slowed so the four-line positioning lands.
// =========================================================================

#[derive(Debug)]
struct IntroScene;
impl Scene for IntroScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(8.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let logo_op = ramp(&frame, 0.2);
        let pulse = 0.5 + 0.5 * (t * 1.5).sin();
        let title_op = ramp(&frame, 0.8);
        let sub_op = ramp(&frame, 1.3);
        let extra_op = ramp(&frame, 2.2);

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
            <g opacity={ramp(&frame, 1.7)}>
                <rect x="870" y="575" width="180" height="36" rx="18" fill={BG_CARD} stroke={ORANGE} stroke-width="1.2" />
                <text x="960" y="599" text-anchor="middle" font-size="16" font-weight="700" fill={ORANGE}>{"v0.4.3 · 全品类工作台"}</text>
            </g>
            <g opacity={extra_op}>
                <rect x="760" y="660" width="400" height="44" rx="22" fill={BG_CARD}
                      stroke={ORANGE_DIM} stroke-width="1" />
                <text x="960" y="690" text-anchor="middle" font-size="20"
                      fill={INK}>{"一句话意图 · 七种出口 · 设备本地"}</text>
            </g>
        </g>)
    }
}

// =========================================================================
// 5. Philosophy (22.0s) — was 3.0s; expanded to land 4 positioning cards
// =========================================================================

#[derive(Debug)]
struct PhilosophyScene;
impl Scene for PhilosophyScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(22.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let items = [
            ("言出法随", "说出口的话,就是可发布的范式"),
            ("意图即创作", "不需要模板,不需要配置,意图即路径"),
            ("本地优先", "你的设备、你的作品、你的模型"),
            ("多宿主真活", "OctoSense 桌面壳 / Rinx / card-host"),
        ];
        let item_svgs: Vec<Svgr> = items.iter().enumerate().map(|(i, (h, d))| {
            let st = 0.4 + i as f32 * 1.6;
            let op = ramp(&frame, st);
            let y = rise(&frame, st);
            // 4 items, 140px spacing fits 350→350+3*140=770 (within 1080 minus top header)
            let base = 340.0 + i as f32 * 140.0;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={Transform::translate(0.0, y)}>
                <rect x="380" y={base} width="1160" height="110" rx="18" fill={BG_CARD} />
                <rect x="380" y={base} width="6" height="110" rx="3" fill={ORANGE} />
                <text x="430" y={base + 50.0} font-size="34" font-weight="700" fill={INK}>{*h}</text>
                <text x="430" y={base + 85.0} font-size="20" fill={INK_SOFT}>{*d}</text>
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let slide = slide_from_right(&frame, 0.3);
        let labels_op = ramp(&frame, 1.0);

        let click_x = 939.0f32;
        let click_y = 523.0f32;
        let click_at = 5.0f32;

        let cursor_x = frame.animate_runtime(AnimateRuntimeInput {
            on_second: 1.5, from: 1300.0, to: click_x, animation_runtime: &EASE_OUT
        });
        let cursor_y = frame.animate_runtime(AnimateRuntimeInput {
            on_second: 1.5, from: 850.0, to: click_y, animation_runtime: &EASE_OUT
        });
        let cursor_op = if t < 1.5 { ramp(&frame, 1.3) } else if t < click_at + 0.4 { 1.0 } else { 1.0 - ramp(&frame, click_at + 0.4) };

        let since = (t - click_at).max(0.0);

        let mut ripple_parts: Vec<Svgr> = Vec::new();
        for k in 0..4 {
            let rp = ((since - k as f32 * 0.07) / 1.0).clamp(0.0, 1.0);
            if rp > 0.0 && rp < 1.0 {
                let rr = rp * 240.0;
                let ro = (1.0 - rp) * 0.75;
                ripple_parts.push(fframes::svgr!(<circle cx={click_x} cy={click_y} r={rr} fill="none" stroke={ORANGE_GLOW} stroke-width="2.5" opacity={ro} />));
            }
        }
        let burst = (1.0 - (since / 0.4).clamp(0.0, 1.0)).max(0.0);
        if burst > 0.0 {
            ripple_parts.push(fframes::svgr!(<circle cx={click_x} cy={click_y} r={10.0 + (1.0-burst)*40.0} fill={ORANGE} opacity={burst * 0.5} />));
        }

        let flash = if since > 0.0 {
            let ft = (since / 0.6).clamp(0.0, 1.0);
            if ft < 0.3 { ft / 0.3 * 0.85 } else { (1.0 - (ft - 0.3) / 0.7) * 0.85 }
        } else { 0.0 };

        let transition = ramp(&frame, 5.6);
        let win_sc = 1.0 - transition * 0.25;
        let win_op = (1.0 - transition).max(0.0);

        let target_op = ramp(&frame, 6.0);
        let target_sc = scale_pop(&frame, 6.0);
        let target_rise = rise(&frame, 6.0);

        // VALUE HOOK — "传统: 90 分钟 → OctoStudio: 3 分钟" appears 7.2s onward
        let hook_op = ramp(&frame, 7.2);
        let hook_x = 240.0;
        let hook_y = 880.0;

        static TAGS: &[(&str, f32, f32)] = &[
            ("原文二创", 200.0, 410.0),
            ("图文文章", 200.0, 475.0),
            ("视频分镜", 200.0, 540.0),
            ("PPT演示", 200.0, 605.0),
            ("拆解视频", 200.0, 670.0),
            ("口播稿", 200.0, 735.0),
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
            // VALUE HOOK — explicit time-savings counter
            <g opacity={hook_op}>
                <rect x={hook_x - 20.0} y={hook_y - 70.0} width="500" height="120" rx="14"
                      fill="#000" opacity="0.7" stroke={ORANGE} stroke-width="1.5" />
                <text x={hook_x + 30.0} y={hook_y - 28.0} font-size="16" fill={INK_SOFT}>
                    {"传统流程"}
                </text>
                <text x={hook_x + 30.0} y={hook_y + 5.0} font-size="32" font-weight="700" fill="#ff6b6b">
                    {"90 分钟"}
                </text>
                <text x={hook_x + 240.0} y={hook_y - 28.0} font-size="16" fill={ORANGE}>
                    {"OctoStudio"}
                </text>
                <text x={hook_x + 240.0} y={hook_y + 5.0} font-size="32" font-weight="700" fill={ORANGE_GLOW}>
                    {"3 分钟"}
                </text>
                <text x={hook_x + 250.0} y={hook_y + 35.0} font-size="13" fill={INK_SOFT}>
                    {"一份稿,一键发布"}
                </text>
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let slide = slide_from_right(&frame, 0.3);

        // 10 文章主题样式(2 行 × 5 列)
        static THEMES: &[(&str, &str)] = &[
            ("不限",     "#666666"),
            ("公众号深度", "#ff6b35"),
            ("干货清单",   "#e8b04b"),
            ("情感散文",   "#6ba6ff"),
            ("小红书种草", "#7fd0a8"),
            ("知乎科普",   "#c98bde"),
            ("科技评测",   "#5fbf9f"),
            ("旅行游记",   "#d68a5c"),
            ("美食探店",   "#e85d75"),
            ("诗歌意象",   "#9ea3c4"),
        ];
        let card_w = 148.0f32;
        let card_h = 60.0f32;
        let gap_x = 12.0f32;
        let gap_y = 16.0f32;
        let x0 = 1080.0f32;
        let y0 = 430.0f32;
        let styles_svgs: Vec<Svgr> = THEMES.iter().enumerate().map(|(i, (name, accent))| {
            let col = (i % 5) as f32;
            let row = (i / 5) as f32;
            let st = 0.4 + i as f32 * 0.08;
            let op = ramp(&frame, st);
            let x = x0 + col * (card_w + gap_x);
            let y = y0 + row * (card_h + gap_y);
            let degrees = -1.0 + (i as f32 * 0.6) - (row * 0.8);
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
                transform={format!("translate({} {}) rotate({})", x, y, degrees)}>
                <rect x="-8" y="-24" width={card_w + 16.0} height={card_h} rx="12" fill="#1a2130" stroke={PIPE} stroke-width="1" />
                <rect x="-8" y="-24" width="6" height={card_h} rx="3" fill={accent} />
                <text x="18" y="6" font-size="15" font-weight="700" fill={INK}>{*name}</text>
            </g>)
        }).collect();

        // 5 视频风格预设(单行)
        static PRESETS: &[(&str, &str)] = &[
            ("电影感", "电影级调色"),
            ("Vlog",   "手持跟拍"),
            ("国风",   "水墨色调"),
            ("赛博",   "霓虹光效"),
            ("治愈",   "柔光暖调"),
        ];
        let p_x0 = 1080.0f32;
        let p_y0 = 620.0f32;
        let p_w = 148.0f32;
        let p_h = 60.0f32;
        let p_gap = 12.0f32;
        let preset_svgs: Vec<Svgr> = PRESETS.iter().enumerate().map(|(i, (name, desc))| {
            let st = 1.4 + i as f32 * 0.08;
            let op = ramp(&frame, st);
            let x = p_x0 + i as f32 * (p_w + p_gap);
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
                transform={format!("translate({} {})", x, p_y0)}>
                <rect x="-8" y="-26" width={p_w + 16.0} height={p_h} rx="12" fill="#141821" stroke={ORANGE} stroke-width="1" opacity="0.85" />
                <text x="0" y="-2" font-size="14" font-weight="700" fill={INK}>{*name}</text>
                <text x="0" y="20" font-size="10" fill={INK_SOFT}>{*desc}</text>
            </g>)
        }).collect();

        // VALUE HOOK — 30:00 countdown timer ("省下 30 分钟构思时间")
        let hook_op = ramp(&frame, 5.5);
        let timer_progress = ((t - 5.5) / 3.0).clamp(0.0, 1.0);
        let secs_left = (30.0 * (1.0 - timer_progress)).max(0.0) as i32;
        let timer_label = if secs_left >= 60 {
            format!("{}:{:02}", secs_left / 60, secs_left % 60)
        } else {
            format!("00:{:02}", secs_left)
        };

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("translate({} 0)", slide)} opacity={ramp(&frame, 0.3)}>
                {desktop_window(80.0, 290.0, 0.95, kenburns(&frame), "02-compose-theme.png")}
            </g>
            <text x="960" y="130" text-anchor="middle" font-size="44" font-weight="700" fill={INK} opacity={ramp(&frame, 0.4)}>{"主题样式 · 视频预设"}</text>
            <text x="960" y="175" text-anchor="middle" font-size="22" fill={INK_SOFT} opacity={ramp(&frame, 0.55)}>{"文章 10 风格 · 视频 5 预设 · 一致语感"}</text>
            <text x="1080" y="395" font-size="18" font-weight="700" fill={ORANGE} opacity={ramp(&frame, 0.4)}>{"文章主题 × 10"}</text>
            <text x="1080" y="585" font-size="18" font-weight="700" fill={ORANGE} opacity={ramp(&frame, 1.4)}>{"视频预设 × 5"}</text>
            {styles_svgs}
            {preset_svgs}
            // VALUE HOOK — countdown timer
            <g opacity={hook_op}>
                <rect x="260" y="800" width="540" height="180" rx="20"
                      fill="#000" opacity="0.75" stroke={ORANGE} stroke-width="2" />
                <text x="290" y="845" font-size="16" fill={INK_SOFT}>
                    {"原来构思时间"}
                </text>
                <text x="290" y="930" font-size="60" font-weight="700" fill="#ff6b6b">
                    {timer_label}
                </text>
                <text x="540" y="900" font-size="20" fill={INK_SOFT}>
                    {"OctoStudio"}
                </text>
                <text x="540" y="935" font-size="32" font-weight="700" fill={ORANGE_GLOW}>
                    {"一句话"}
                </text>
                <rect x="290" y="945" width={480.0 * timer_progress} height="6" rx="3"
                      fill={ORANGE_GLOW} opacity="0.85" />
            </g>
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(8.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let slide = slide_from_left(&frame, 0.3);
        let text_op = ramp(&frame, 1.2);
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let rise_y = rise(&frame, 0.3);
        let text_op = ramp(&frame, 1.2);
        let strip_op = ramp(&frame, 1.0);
        let t = frame.seconds();

        static TAGS: &[(&str, f32, f32)] = &[
            ("分镜表", 200.0, 380.0),
            ("合成建议", 200.0, 460.0),
            ("SRT字幕", 200.0, 540.0),
        ];
        let tag_svgs = tech_tags(t, TAGS);

        // 5 视频风格预设 chips(右上)
        static PRESETS: &[(&str, &str)] = &[
            ("电影感", "#ff6b35"),
            ("Vlog",   "#e8b04b"),
            ("国风",   "#7fd0a8"),
            ("赛博",   "#6ba6ff"),
            ("治愈",   "#c98bde"),
        ];
        let chip_w = 130.0f32;
        let chip_h = 36.0f32;
        let chip_gap = 8.0f32;
        let preset_svgs: Vec<Svgr> = PRESETS.iter().enumerate().map(|(i, (name, accent))| {
            let st = 1.5 + i as f32 * 0.12;
            let op = ramp(&frame, st);
            let x = 1300.0 + (i as f32 - 2.0) * (chip_w + chip_gap);
            let y = 280.0;
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op}
                transform={format!("translate({} {})", x, y)}>
                <rect x="-4" y="-18" width={chip_w} height={chip_h} rx="18" fill="#141821" stroke={accent} stroke-width="1" />
                <circle cx="12" cy="-0" r="4" fill={accent} />
                <text x="26" y="5" font-size="14" font-weight="700" fill={INK}>{*name}</text>
            </g>)
        }).collect();

        let cell_w = 130.0f32;
        let cell_h = 74.0f32;
        let pitch = 152.0f32;
        let strip_x = 200.0f32;
        let strip_y = 830.0f32;
        let sweep = ((t - 1.0) / 2.5).clamp(0.0, 1.0);
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
                <text x="200" y="275" font-size="20" fill={INK_SOFT}>{"转场 · 配乐 · 调色 · 5 种风格预设"}</text>
            </g>
            {tag_svgs}
            <text x="1300" y="248" text-anchor="middle" font-size="14" font-weight="700" fill={ORANGE} opacity={ramp(&frame, 1.4)}>{"风格预设 × 5"}</text>
            {preset_svgs}
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
// 10. EditPanel (8.0s) — uses the previously-unused 04-edit-panel.png
// =========================================================================

#[derive(Debug)]
struct EditPanelScene;
impl Scene for EditPanelScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(8.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let slide = slide_from_left(&frame, 0.3);
        let head_op = ramp(&frame, 1.0);

        // VALUE HOOK — "改的就是 AI 出的稿" hint at top of edit panel
        let hint_op = ramp(&frame, 3.0);

        // Highlighter sweep across one of the editable items
        let sweep_p = ((t - 4.0) / 2.5).clamp(0.0, 1.0);

        // "改一改" affordance — a +/– button group at bottom-right
        let afford_op = ramp(&frame, 1.5);

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <g transform={format!("translate({} 0)", slide)} opacity={ramp(&frame, 0.3)}>
                {desktop_window(150.0, 280.0, 0.95, kenburns(&frame), "04-edit-panel.png")}
            </g>
            <g opacity={head_op}>
                <text x="1200" y="180" font-size="38" font-weight="700" fill={INK}>{"通用计划编辑器"}</text>
                <text x="1200" y="225" font-size="20" fill={INK_SOFT}>{"改一改,就是你的稿"}</text>
            </g>
            // Hint badge — orange "AI 出 plan · 你来定稿"
            <g opacity={hint_op}>
                <rect x="1180" y="260" width="500" height="46" rx="22"
                      fill={ORANGE} opacity="0.92" />
                <text x="1200" y="290" font-size="18" font-weight="700" fill="#000">
                    {"AI 出 plan · 你来定稿"}
                </text>
            </g>
            // Highlighter sweep across the panel
            {if sweep_p > 0.0 && sweep_p < 1.0 {
                let sw = sweep_p * 800.0;
                fframes::svgr!(<rect x="200" y="780" width={sw} height="44" rx="6"
                    fill={ORANGE_GLOW} opacity="0.35" />)
            } else { fframes::svgr!(<g />) }}
            // Affordance group (+/–  ↑↓ buttons) at bottom-right
            <g opacity={afford_op} transform={format!("translate(1640 {})", 540.0 + (t * 0.8).sin() * 4.0)}>
                <rect x="0" y="0" width="220" height="80" rx="14" fill={BG_CARD}
                      stroke={ORANGE} stroke-width="1.5" />
                <text x="20" y="32" font-size="14" fill={INK_SOFT}>{"编辑条目"}</text>
                <g font-size="22" font-weight="700">
                    <circle cx="30" cy="58" r="14" fill={ORANGE} />
                    <text x="30" y="65" text-anchor="middle" fill="#000">{"↑"}</text>
                    <circle cx="76" cy="58" r="14" fill={ORANGE} />
                    <text x="76" y="65" text-anchor="middle" fill="#000">{"↓"}</text>
                    <circle cx="130" cy="58" r="14" fill={ORANGE} />
                    <text x="130" y="65" text-anchor="middle" fill="#000">{"+"}</text>
                    <circle cx="184" cy="58" r="14" fill="#ff6b6b" />
                    <text x="184" y="65" text-anchor="middle" fill="#000">{"×"}</text>
                </g>
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 11. MoreFormats (now also Export, 10.0s) — 7 export formats, was 3.0s
// =========================================================================

#[derive(Debug)]
struct MoreFormatsScene;
impl Scene for MoreFormatsScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(5.0) }
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(5.0) }
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
// 11. Export (3.5s) — 7 种格式按场景动态出现
// =========================================================================

#[derive(Debug)]
struct ExportScene;
impl Scene for ExportScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
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
// 12. M3Content (4.0s) — 搜索 / 标签 / AI 历史 内容管理三件套(v0.3.5)
// =========================================================================

#[derive(Debug)]
struct M3ContentScene;
impl Scene for M3ContentScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);

        // Left phone: plaza with tags (m3-tags.png)
        let left_op = ramp(&frame, 0.35);
        let left_sc = scale_pop(&frame, 0.35);
        // Center phone: plaza with search active (m3-search.png) + typing-in-progress
        let center_op = ramp(&frame, 0.55);
        let center_sc = scale_pop(&frame, 0.55);
        // Right phone: AI history card (synthesized SVG)
        let right_op = ramp(&frame, 0.75);
        let right_sc = scale_pop(&frame, 0.75);

        let ph_w = 480.0; let ph_h = 540.0;
        let lx = 80.0;  let ly = 240.0;
        let cx = 720.0;  let cy = 200.0;
        let rx = 1360.0; let ry = 240.0;

        let search_t = frame.seconds();
        let type_progress = ((search_t - 1.5) / 1.5).clamp(0.0, 1.0);
        static QUERY: &[&str] = &["日", "落", "日 落", "日 落 之", "日 落 之 城", "日 落 之 城 ·"];
        let typed: &str = QUERY[(type_progress * (QUERY.len() as f32 - 0.01)) as usize];

        // Build search overlay group separately (svgr macro can't host if-else)
        let search_overlay: Svgr = if type_progress > 0.0 {
            fframes::svgr!(<g>
                <rect x="32" y="442" width="416" height="42" rx="21" fill="#fff" stroke={ORANGE} stroke-width="2" />
                <text x="48" y="468" font-size="16" fill="#1c1c1e" font-family="Arial Unicode MS">{typed}<tspan fill={ORANGE}>{"▏"}</tspan></text>
            </g>)
        } else {
            fframes::svgr!(<g />)
        };

        // Build AI history card body separately
        static HIST: &[(&str, &str)] = &[
            ("起标题",   "海浪不着急,所以每天都好看"),
            ("风格迁移", "散文 / 干货 / 小红书 / 知乎"),
            ("摘要",     "把三天假期过成一个月..."),
        ];
        let mut hist_parts: Vec<Svgr> = Vec::new();
        hist_parts.push(fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <rect x="24" y="24" width={ph_w - 48.0} height="60" rx="10" fill="#fff" stroke="#f0d9c4" stroke-width="1" />
            <text x="40" y="50" font-size="14" font-weight="700" fill="#1c1c1e">{"AI 历史 · 最近 3 条"}</text>
            <text x="40" y="72" font-size="11" fill="#8e8e93">{"夏日海边慢生活"}</text>
        </g>));
        for (i, (label, body)) in HIST.iter().enumerate() {
            let yy = 100.0 + i as f32 * 90.0;
            hist_parts.push(fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
                <rect x="24" y={yy} width={ph_w - 48.0} height="78" rx="10" fill="#fff" stroke="#e6dccf" stroke-width="1" />
                <rect x="24" y={yy} width="4" height="78" rx="2" fill="#ff6b35" />
                <text x="40" y={yy + 22.0} font-size="12" font-weight="700" fill="#ff6b35">{*label}</text>
                <text x="40" y={yy + 42.0} font-size="12" fill="#1c1c1e">{*body}</text>
                <text x="40" y={yy + 62.0} font-size="10" fill="#8e8e93">{"↳ class:fast · 已存"}</text>
            </g>));
        }

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK} opacity={title_op}>{"M3 · 内容管理三件套"}</text>
            <text x="960" y="160" text-anchor="middle" font-size="19" fill={INK_SOFT} opacity={title_op}>{"搜索 · 标签 · AI 历史 · 全部本地"}</text>

            // Left: plaza + tags
            <g opacity={left_op} transform={format!("translate({} {}) scale({})", lx + ph_w/2.0, ly + ph_h/2.0, left_sc * 0.78)}>
                <g transform={format!("translate({} {})", -ph_w/2.0, -ph_h/2.0)}>
                    <rect x="-8" y="-8" width={ph_w + 16.0} height={ph_h + 16.0} rx="34" fill="#080808" />
                    <rect x="0" y="0" width={ph_w} height={ph_h} rx="28" fill="#faf6f1" />
                    <image href="m3-tags.png" x="0" y="0" width={ph_w} height={ph_h} preserveAspectRatio="xMidYMid slice" />
                </g>
            </g>
            <text x={lx + ph_w*0.78/2.0} y={690.0} text-anchor="middle" font-size="16" font-weight="700" fill={ORANGE} opacity={left_op}>{"① 标签筛选 · 按 #chip 过滤作品"}</text>

            // Center: search + live typing
            <g opacity={center_op} transform={format!("translate({} {}) scale({})", cx + ph_w/2.0, cy + ph_h/2.0, center_sc * 0.78)}>
                <g transform={format!("translate({} {})", -ph_w/2.0, -ph_h/2.0)}>
                    <rect x="-8" y="-8" width={ph_w + 16.0} height={ph_h + 16.0} rx="34" fill="#080808" />
                    <rect x="0" y="0" width={ph_w} height={ph_h} rx="28" fill="#faf6f1" />
                    <image href="m3-search.png" x="0" y="0" width={ph_w} height={ph_h} preserveAspectRatio="xMidYMid slice" />
                    {search_overlay}
                </g>
            </g>
            <text x={cx + ph_w*0.78/2.0} y={690.0} text-anchor="middle" font-size="16" font-weight="700" fill={ORANGE} opacity={center_op}>{"② 广场搜索 · 即时过滤标题/原文"}</text>

            // Right: AI history (synthesized SVG)
            <g opacity={right_op} transform={format!("translate({} {}) scale({})", rx + ph_w/2.0, ry + ph_h/2.0, right_sc * 0.78)}>
                <g transform={format!("translate({} {})", -ph_w/2.0, -ph_h/2.0)}>
                    <rect x="-8" y="-8" width={ph_w + 16.0} height={ph_h + 16.0} rx="34" fill="#080808" />
                    <rect x="0" y="0" width={ph_w} height={ph_h} rx="28" fill="#faf6f1" />
                    {hist_parts}
                </g>
            </g>
            <text x={rx + ph_w*0.78/2.0} y={690.0} text-anchor="middle" font-size="16" font-weight="700" fill={ORANGE} opacity={right_op}>{"③ AI 历史 · 每篇最近 3 条"}</text>

            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 13. AiAssistant (3.5s) — model 助手 7 项(v0.4.0)
// =========================================================================

#[derive(Debug)]
struct AiAssistantScene;
impl Scene for AiAssistantScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(10.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let _panel_op = ramp(&frame, 0.4);
        let _panel_sc = scale_pop(&frame, 0.4);

        // 7 项 AI 助手
        static ITEMS: &[(&str, &str, &str)] = &[
            ("起标题",     "schema:title",  "auto"),
            ("关键词",     "{keywords:[]}", "fast"),
            ("摘要",       "{summary}",     "fast"),
            ("风格迁移",   "4 风格",        "fast"),
            ("中英对照",   "{zh,en}",       "fast"),
            ("标题打分",   "0-100",         "fast"),
            ("模型预算",   "今日剩余",       "sys"),
        ];

        let card_w = 232.0f32;
        let card_h = 130.0f32;
        let gap_x = 18.0f32;
        let gap_y = 22.0f32;
        let x0 = (1920.0 - (card_w * 4.0 + gap_x * 3.0)) / 2.0;
        let y0 = 350.0f32;

        let card_svgs: Vec<Svgr> = ITEMS.iter().enumerate().map(|(i, (name, schema, cls))| {
            let col = (i % 4) as f32;
            let row = (i / 4) as f32;
            let st = 0.4 + i as f32 * 0.06;
            let op = ramp(&frame, st);
            let sc = scale_pop(&frame, st);
            let x = x0 + col * (card_w + gap_x);
            let y = y0 + row * (card_h + gap_y);
            fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT} opacity={op} transform={format!("translate({} {}) scale({})", x, y, sc)}>
                <rect width={card_w} height={card_h} rx="14" fill={BG_CARD} stroke={ORANGE} stroke-width="1.2" />
                <rect x="0" y="0" width="6" height={card_h} rx="3" fill={ORANGE} />
                <text x="22" y="28" font-size="11" letter-spacing="2" fill={ORANGE}>{"AI ASSISTANT"}</text>
                <text x="22" y="60" font-size="22" font-weight="700" fill={INK}>{*name}</text>
                <text x="22" y="86" font-size="11" fill={INK_SOFT}>{*schema}</text>
                <rect x="22" y={card_h - 30.0} width="50" height="18" rx="9" fill="#141821" stroke={PIPE} stroke-width="1" />
                <text x="47" y={card_h - 18.0} text-anchor="middle" font-size="10" fill={INK_SOFT}>{*cls}</text>
            </g>)
        }).collect();

        // 右侧合成示意:输入原文 → 一键调用 → 输出
        let flow_op = ramp(&frame, 1.2);
        let out_x = 280.0f32;

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            <text x="960" y="120" text-anchor="middle" font-size="42" font-weight="700" fill={INK} opacity={title_op}>{"AI 助手 · 工坊屏 7 项"}</text>
            <text x="960" y="160" text-anchor="middle" font-size="19" fill={INK_SOFT} opacity={title_op}>{"一键调用 model.complete · schema 强制 · fast 类"}</text>

            {card_svgs}

            <g opacity={flow_op}>
                <text x="960" y="700" text-anchor="middle" font-size="16" fill={INK_SOFT}>{"原文 / 一句话 → 一键调用 → 结构化输出,可直接填回计划编辑器"}</text>
                <g transform={format!("translate({} 770)", out_x)}>
                    <rect x="0" y="0" width="380" height="56" rx="10" fill={BG_CARD} stroke={PIPE} stroke-width="1" />
                    <text x="20" y="24" font-size="13" fill={ORANGE}>{"原文"}</text>
                    <text x="20" y="46" font-size="13" fill={INK_SOFT}>{"你有多久没看完一场日落了"}</text>
                </g>
                <g transform={format!("translate({} 770)", out_x + 410.0)}>
                    <rect x="0" y="0" width="60" height="56" rx="10" fill={ORANGE} opacity="0.2" />
                    <text x="30" y="34" text-anchor="middle" font-size="22" fill={ORANGE}>{"→"}</text>
                </g>
                <g transform={format!("translate({} 770)", out_x + 490.0)}>
                    <rect x="0" y="0" width="60" height="56" rx="10" fill={ORANGE} />
                    <text x="30" y="38" text-anchor="middle" font-size="18" font-weight="700" fill="#fff">{"AI"}</text>
                </g>
                <g transform={format!("translate({} 770)", out_x + 570.0)}>
                    <rect x="0" y="0" width="60" height="56" rx="10" fill={ORANGE} opacity="0.2" />
                    <text x="30" y="34" text-anchor="middle" font-size="22" fill={ORANGE}>{"→"}</text>
                </g>
                <g transform={format!("translate({} 770)", out_x + 650.0)}>
                    <rect x="0" y="0" width="380" height="56" rx="10" fill={BG_CARD} stroke={ORANGE} stroke-width="1.5" />
                    <text x="20" y="24" font-size="13" fill={ORANGE}>{"输出"}</text>
                    <text x="20" y="46" font-size="13" fill={INK}>{"8 个候选标题 · 推荐指数 · 一键填入"}</text>
                </g>
            </g>

            <rect width="1920" height="1080" fill={BG} opacity={dip_in(&frame)} />
        </g>)
    }
}

// =========================================================================
// 14. HostsIntro (1.5s) — quick three-host triptych
// =========================================================================

#[derive(Debug)]
struct HostsIntroScene;
impl Scene for HostsIntroScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(5.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.15);
        // Three glass cards arranged in a fan, each labeled with a host name
        let labels = ["card-host", "OctoSense 桌面壳", "Rinx 小程序"];
        let cards: Vec<Svgr> = (0..3usize).map(|i| {
            let st = 0.25 + i as f32 * 0.12;
            let op = ramp(&frame, st);
            let sc = scale_pop(&frame, st);
            let deg = -12.0 + i as f32 * 12.0;
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let shell_op = ramp(&frame, 0.4);
        let shell_sc = scale_pop(&frame, 0.4);
        let badge_op = ramp(&frame, 1.2);
        let cap_op   = ramp(&frame, 2.0);

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

            // AI 已连接 badge — 右下角
            <g opacity={badge_op} transform={format!("translate({} {})", cx + win_w * 0.36, cy + win_h * 0.45)}>
                <rect x="-12" y="-22" width="170" height="44" rx="22" fill="#0a0c11" stroke="#3aa867" stroke-width="1.5" />
                <circle cx="6" cy="0" r="6" fill="#3aa867" />
                <circle cx="6" cy="0" r="3" fill="#fff" />
                <text x="22" y="6" font-size="14" font-weight="700" fill="#7fd0a8">{"AI 已连接"}</text>
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let title_op = ramp(&frame, 0.2);
        let mob_op = ramp(&frame, 0.35);
        let tab_op = ramp(&frame, 0.55);
        let dsk_op = ramp(&frame, 0.75);
        let mob_sc = scale_pop(&frame, 0.35);
        let tab_sc = scale_pop(&frame, 0.55);
        let dsk_sc = scale_pop(&frame, 0.75);
        let badge_op = ramp(&frame, 1.4);

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

            // AI 仅声明 badge — 中下
            <g opacity={badge_op} transform={format!("translate({} {})", 960.0, 940.0)}>
                <rect x="-180" y="-22" width="360" height="44" rx="22" fill="#0a0c11" stroke="#c98bde" stroke-width="1.5" />
                <circle cx="-156" cy="0" r="6" fill="#c98bde" />
                <text x="-138" y="6" font-size="14" font-weight="700" fill="#c98bde">{"card-host · AI 仅声明,未连接"}</text>
            </g>

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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
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
// 22. PlatformsFuture (7.0s) — 6 platforms + Web, "Coming Soon" roadmap
// =========================================================================

#[derive(Debug)]
struct PlatformsFutureScene;
impl Scene for PlatformsFutureScene {
    fn duration(&self) -> Duration<'_> { Duration::Seconds(7.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let head_op = ramp(&frame, 0.3);
        let subhead_op = ramp(&frame, 0.8);

        // Three category columns: 桌面 Desktop / 移动 Mobile / Web
        // Each platform is a colored chip with its brand color
        let categories: &[(&str, &[(&str, &str)])] = &[
            ("桌面 Desktop", &[
                ("Windows", "#00a1f1"),
                ("macOS",   "#a2aaad"),
                ("Linux",   "#fcc624"),
            ]),
            ("移动 Mobile", &[
                ("Android", "#3ddc84"),
                ("iOS",     "#007aff"),
                ("鸿蒙",     "#ff6b35"),
            ]),
            ("Web", &[
                ("Browser", "#4285f4"),
            ]),
        ];

        let cat_x: [f32; 3] = [300.0, 960.0, 1620.0];
        let cat_y_header = 380.0;
        let chip_y_start = 480.0;
        let chip_w = 180.0;
        let chip_h = 56.0;
        let chip_gap = 14.0;

        let mut cat_svgs: Vec<Svgr> = Vec::new();
        for (ci, (cat_name, platforms)) in categories.iter().enumerate() {
            let cx = cat_x[ci];
            let head_st = 0.4 + ci as f32 * 0.15;
            let head_op_c = ramp(&frame, head_st);
            cat_svgs.push(fframes::svgr!(<g opacity={head_op_c} font-family={FONT} font-weight={WEIGHT}>
                <text x={cx} y={cat_y_header} text-anchor="middle" font-size="26"
                      letter-spacing="2" fill={ORANGE}>{*cat_name}</text>
            </g>));
            for (pi, (plat_name, color)) in platforms.iter().enumerate() {
                let chip_st = 0.9 + ci as f32 * 0.15 + pi as f32 * 0.18;
                let chip_op = ramp(&frame, chip_st);
                let slide_y = rise(&frame, chip_st);
                let chip_x = cx - chip_w / 2.0;
                let chip_y = chip_y_start + pi as f32 * (chip_h + chip_gap) + slide_y;
                cat_svgs.push(fframes::svgr!(<g opacity={chip_op} font-family={FONT} font-weight={WEIGHT}
                    transform={format!("translate(0 {})", slide_y)}>
                    <rect x={chip_x} y={chip_y} width={chip_w} height={chip_h} rx="14"
                          fill={color} />
                    <rect x={chip_x} y={chip_y} width={chip_w} height={chip_h} rx="14"
                          fill="none" stroke="#fff" stroke-width="1.5" opacity="0.7" />
                    <text x={cx} y={chip_y + 36.0} text-anchor="middle" font-size="22"
                          font-weight="700" fill="#fff">{*plat_name}</text>
                </g>));
            }
        }

        // Header "未来可运行在..." at top
        let header_y = 160.0;
        // Subhead "COMING SOON" tag below header
        let tag_op = ramp(&frame, 1.5);

        // Connecting lines from center hub to platforms
        let hub_x = 960.0;
        let hub_y = 220.0;
        let hub_pulse = 0.5 + 0.5 * (t * 1.5).sin();
        let mut lines: Vec<Svgr> = Vec::new();
        for ci in 0..3 {
            let cx = cat_x[ci];
            let line_op = ramp(&frame, 1.0 + ci as f32 * 0.2) * 0.35;
            lines.push(fframes::svgr!(<line x1={hub_x} y1={hub_y + 30.0}
                x2={cx} y2={cat_y_header - 12.0}
                stroke={ORANGE} stroke-width="1" opacity={line_op} />));
        }

        fframes::svgr!(<g font-family={FONT} font-weight={WEIGHT}>
            // Header
            <g opacity={head_op}>
                <text x="960" y={header_y} text-anchor="middle" font-size="44"
                      font-weight="700" fill={INK}>{"未来可运行在"}</text>
            </g>
            // Subhead / brand stamp
            <g opacity={subhead_op}>
                <rect x="850" y={header_y + 30.0} width="220" height="34" rx="17"
                      fill={ORANGE_DIM} stroke={ORANGE} stroke-width="1.2" />
                <text x="960" y={header_y + 52.0} text-anchor="middle" font-size="14"
                      letter-spacing="4" fill={ORANGE}>{"ROADMAP · COMING SOON"}</text>
            </g>
            // Connecting lines from hub
            {lines}
            // Center hub (OctoStudio logo small)
            <g opacity={head_op}>
                <circle cx={hub_x} cy={hub_y + 30.0} r="36" fill={ORANGE} opacity={0.15 + hub_pulse * 0.15} />
                <circle cx={hub_x} cy={hub_y + 30.0} r="22" fill={ORANGE} opacity="0.9" />
                <text x={hub_x} y={hub_y + 38.0} text-anchor="middle" font-size="18"
                      font-weight="700" fill="#fff">{"OS"}</text>
            </g>
            // Three category columns
            {cat_svgs}
            // Bottom value strip — "同一份 bundle,处处可跑"
            <g opacity={tag_op}>
                <rect x="660" y="900" width="600" height="44" rx="22"
                      fill={BG_CARD} stroke={ORANGE} stroke-width="1.2" />
                <text x="960" y="930" text-anchor="middle" font-size="18"
                      fill={INK}>{"同一份 bundle · 处处可跑"}</text>
            </g>
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
    fn duration(&self) -> Duration<'_> { Duration::Seconds(13.0) }
    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let logo_op = ramp(&frame, 0.2);
        let title_op = ramp(&frame, 0.45);
        let slogan_op = ramp(&frame, 0.75);
        let info_op = ramp(&frame, 8.0);
        let fade = dip_out(&frame, 12.0);

        // VALUE HOOK — "2:00:00 → 0:03:00" timer appears from 3.0s onward
        let timer_op = ramp(&frame, 3.0);
        let timer_progress = ((t - 3.0) / 5.0).clamp(0.0, 1.0);
        // Animate from 2:00:00 (7200s) to 0:03:00 (180s), where progress 1.0 = at target
        let total_secs_start: f32 = 7200.0;
        let total_secs_end: f32 = 180.0;
        let current_secs = total_secs_start * (1.0 - timer_progress) + total_secs_end * timer_progress;
        let mm = ((current_secs % 3600.0) / 60.0) as i32;
        let ss = (current_secs % 60.0) as i32;

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
            // VALUE HOOK — 2:00:00 → 0:03:00 timer, bottom-left
            <g opacity={timer_op}>
                <rect x="80" y="820" width="640" height="160" rx="20"
                      fill="#000" opacity="0.78" stroke={ORANGE} stroke-width="2" />
                <text x="110" y="860" font-size="16" fill={INK_SOFT}>
                    {"传统流程(同输入)"}
                </text>
                <text x="110" y="935" font-size="48" font-weight="700" fill="#ff6b6b"
                      opacity={(1.0 - timer_progress * 0.7).max(0.3)}>
                    {format!("{:02}:{:02}:{:02}", 2, 0, 0)}
                </text>
                <text x="110" y="965" font-size="13" fill={INK_SOFT}>
                    {"构思 + 写作 + 排版 + 复制 4 个平台"}
                </text>
                <text x="400" y="860" font-size="16" fill={ORANGE}>
                    {"OctoStudio"}
                </text>
                <text x="400" y="935" font-size="48" font-weight="700" fill={ORANGE_GLOW}>
                    {format!("{:02}:{:02}:{:02}", 0, mm, ss)}
                </text>
                <text x="400" y="965" font-size="13" fill={INK_SOFT}>
                    {"一句话 · 一处编辑 · 多平台同时出"}
                </text>
                <rect x="110" y="975" width="540" height="4" rx="2" fill={ORANGE_DIM} />
                <rect x="110" y="975" width={540.0 * timer_progress} height="4" rx="2"
                      fill={ORANGE_GLOW} />
            </g>
            <ellipse cx={sweep_cx} cy={line_y - 18.0} rx="70" ry="42" fill="#fff" opacity={sweep_op * 0.5} />
            <ellipse cx={sweep_cx} cy={line_y - 18.0} rx="30" ry="20" fill="#fff" opacity={sweep_op} />
            {up_particles}
            <rect x={start_x - 30.0} y={line_y + 22.0} width={total_w + 60.0} height="3" fill={ORANGE} opacity={0.55 * ramp(&frame, 2.1)} />
            <rect x={start_x - 30.0} y={line_y + 22.0} width={total_w + 60.0} height="8" fill={ORANGE} opacity={0.18 * ramp(&frame, 2.1)} />
            <g opacity={info_op}>
                <text x="960" y="710" text-anchor="middle" font-size="15" fill={INK_SOFT}>{"Rinx · OctoSense App Hub · Apache-2.0 · v0.4.3"}</text>
                <text x="960" y="740" text-anchor="middle" font-size="13" fill={ORANGE} opacity="0.85">{"10 主题 · 5 视频预设 · M3 内容管理 · AI 助手 7 项"}</text>
                // GitHub URL — prominent, brand orange, appears with the rest of the
                // info block so the viewer has a clear "where to find the source" hook.
                <g transform="translate(960 800)">
                    <rect x="-260" y="-26" width="520" height="52" rx="26"
                          fill={ORANGE} opacity="0.92" />
                    <rect x="-260" y="-26" width="520" height="52" rx="26"
                          fill="none" stroke={ORANGE_GLOW} stroke-width="1.5" />
                    <text x="-238" y="8" font-size="22" font-weight="700" fill="#fff">{"↗"}</text>
                    <text x="0" y="8" text-anchor="middle" font-size="24" font-weight="700"
                          letter-spacing="1" fill="#fff">{"github.com/aios-pub/OctoStudio"}</text>
                </g>
            </g>
            <rect width="1920" height="1080" fill={BG} opacity={fade} />
        </g>)
    }
}
