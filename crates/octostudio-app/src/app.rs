//! App shell — widget tree, app entry, 6-screen navigation.
//!
//! C3 inlines the 6 screen widget templates here because makepad's
//! `script_mod!` macro uses lexical scope — widgets defined in one
//! crate's `script_mod!` block cannot be referenced from another
//! crate's `startup()` body, even if both are called on the same
//! `ScriptVm`. The render crate still owns the typed [`AppState`] +
//! Rust helpers; the visual tree lives in this file.
//!
//! State is held in a process-global [`APP_STATE`] (same pattern as
//! makepad's `todo` example uses for `static TODOS: RwLock<...>`).

use std::sync::{Mutex, MutexGuard, OnceLock};

use makepad_widgets::*;
use octostudio_core::Screen;
use octostudio_render::AppState;

use crate::boot;

static APP_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

fn state() -> MutexGuard<'static, AppState> {
    APP_STATE
        .get_or_init(|| Mutex::new(AppState::new()))
        .lock()
        .expect("APP_STATE mutex poisoned")
}

script_mod! {
    use mod.prelude.widgets.*

    // ---------- 6 screen widgets ----------

    let HomeView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        home_title := Label{
            text: "广场"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        home_subtitle := Label{
            text: "今天共 N 件作品 · C6 配图测试就位"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        home_test_image := ButtonFlat{
            text: "🎨 测试 AI 配图(需 API key)"
            height: 36 padding: Inset{left: 14, right: 14}
            draw_bg +: {
                color: #xFF6B35
                color_hover: #xFF8866
                border_radius: 18.0
            }
            draw_text +: {
                color: #xFFFFFF
            }
        }
        home_test_video := ButtonFlat{
            text: "🎬 测试 AI 合成视频(需 API key,等 30s)"
            height: 36 padding: Inset{left: 14, right: 14}
            draw_bg +: {
                color: #x4A6FA5
                color_hover: #x6A8FC5
                border_radius: 18.0
            }
            draw_text +: {
                color: #xFFFFFF
            }
        }
        home_placeholder := Label{
            text: "works 列表 / 搜索 / 排序 chip / 场景网格 — 后续 commit"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let ComposeView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        compose_title := Label{
            text: "开始一段创作"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        compose_subtitle := Label{
            text: "挑一个场景,粘贴原文,一句话就够 — C3 stub"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        compose_placeholder := Label{
            text: "10 场景 chips + 主题/预设 chips + 3 Card 输入 + CTA — C5 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let StudioView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        studio_title := Label{
            text: "工坊"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        studio_subtitle := Label{
            text: "8 改写风格 + 7 AI 助手 + 流式 chat — C3 stub"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        studio_placeholder := Label{
            text: "style chips / 撤销 / 标题 / 原文 / 成稿 / AI 助手 panel — C5-C9 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let PlanView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        plan_title := Label{
            text: "计划"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        plan_subtitle := Label{
            text: "条目编辑 + AI 配图 + AI 合成视频 — C3 stub"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        plan_placeholder := Label{
            text: "条目 RoundedView 列表 / 视频合成建议 / 配图开关 — C6-C7 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let ExportView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        export_title := Label{
            text: "导出"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        export_subtitle := Label{
            text: "8 格式(markdown/wechat/notion/srt/pack/marp/markmap) — C3 stub"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        export_placeholder := Label{
            text: "格式网格 + 大预览 Card + 复制按钮 — C8 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let SettingsView = View{
        width: Fill height: Fill
        flow: Down
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        spacing: 12
        draw_bg.color: #xF6F6F8

        settings_title := Label{
            text: "设置"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        settings_subtitle := Label{
            text: "API key / 测试连接 / 主题 / 配额 — C3 stub"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        settings_placeholder := Label{
            text: "API key TextInput + 测试连接 button + 主题 picker + 配额 Card — C9 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    // ---------- 5-tab bottom bar ----------
    // Each tab is a ButtonFlat with a single "icon label" line. The icon
    // is drawn larger via inline font_size; the label is the second line.
    // We rely on `set_text()` from Rust to keep the 5 tabs distinct and
    // on `clicked(actions)` in MatchEvent to detect taps (View doesn't
    // have a built-in click action; ButtonFlat does).

    let TabBtn = ButtonFlat{
        width: Fill height: 56
        text: "◐\n广场"
        draw_bg +: {
            color: #xF6F6F8
            border_radius: 10.0
            color_hover: #xE5E5EA
        }
        draw_text +: {
            color: #x8E8E93
        }
    }

    let TabBar = View{
        width: Fill height: 60
        flow: Right
        padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
        spacing: 4
        align: Align{y: 0.5}
        draw_bg.color: #xFFFFFF
        draw_bg.border_size: 1.0
        draw_bg.border_color: #xE5E5EA

        tab_home := TabBtn{}
        tab_compose := TabBtn{}
        tab_studio := TabBtn{}
        tab_plan := TabBtn{}
        tab_settings := TabBtn{}
    }

    let AppHeader = View{
        width: Fill height: Fit
        flow: Down
        spacing: 4
        padding: Inset{top: 12, bottom: 12, left: 24, right: 24}
        draw_bg.color: #xFFFFFF

        title := Label{
            text: "OctoStudio v0.6"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        subtitle := Label{
            text: "Native Makepad · 6 屏 + 真实 Agnes AI · C3 渲染骨架就位"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    let AppFooter = View{
        width: Fill height: Fit
        flow: Right
        spacing: 8
        padding: Inset{top: 6, bottom: 6, left: 24, right: 24}
        align: Align{y: 0.5}
        draw_bg.color: #xF6F6F8

        status_label := Label{
            width: Fill
            text: "就绪 · 6 屏 + Settings + 底部 TabBar 渲染完成"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "OctoStudio v0.6 (native)"
                window.inner_size: vec2(1280, 800)
                pass +: { clear_color: #xFFFFFF }
                body +: {
                    View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 0
                        draw_bg.color: #xFFFFFF

                        AppHeader{}

                        screens := View{
                            width: Fill height: Fill
                            flow: Overlay
                            draw_bg.color: #xFFFFFF

                            home_view     := HomeView     { visible: true }
                            compose_view  := ComposeView  { visible: false }
                            studio_view   := StudioView   { visible: false }
                            plan_view     := PlanView     { visible: false }
                            export_view   := ExportView   { visible: false }
                            settings_view := SettingsView { visible: false }
                        }

                        AppFooter{}
                        TabBar{}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl App {
    /// Goto — flip the visible flag of each screen, then request a redraw.
    fn goto(&mut self, cx: &mut Cx, mode: Screen) {
        state().mode = mode;
        self.ui.view(cx, ids!(home_view))
            .set_visible(cx, mode == Screen::Home);
        self.ui.view(cx, ids!(compose_view))
            .set_visible(cx, mode == Screen::Compose);
        self.ui.view(cx, ids!(studio_view))
            .set_visible(cx, mode == Screen::Studio);
        self.ui.view(cx, ids!(plan_view))
            .set_visible(cx, mode == Screen::Plan);
        self.ui.view(cx, ids!(export_view))
            .set_visible(cx, mode == Screen::Export);
        self.ui.view(cx, ids!(settings_view))
            .set_visible(cx, mode == Screen::Settings);
        self.ui.redraw(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // C4: hydrate AppState from disk.
        let (works, config) = boot::boot();
        {
            let mut s = state();
            s.works = works;
            s.api_key = config.api_key;
            // theme_mode + model_class are read from config but only
            // consumed by the Settings screen (C9) and AI calls (C5).
        }
        log!("OctoStudio v0.6 — native Makepad app starting (C4 boot + storage)");
        self.init_tabs(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(tab_home)).clicked(actions) {
            self.goto(cx, Screen::Home);
            return;
        }
        if self.ui.button(cx, ids!(tab_compose)).clicked(actions) {
            self.goto(cx, Screen::Compose);
            return;
        }
        if self.ui.button(cx, ids!(tab_studio)).clicked(actions) {
            self.goto(cx, Screen::Studio);
            return;
        }
        if self.ui.button(cx, ids!(tab_plan)).clicked(actions) {
            self.goto(cx, Screen::Plan);
            return;
        }
        if self.ui.button(cx, ids!(tab_settings)).clicked(actions) {
            self.goto(cx, Screen::Settings);
            return;
        }
        if self.ui.button(cx, ids!(home_test_image)).clicked(actions) {
            self.test_image(cx);
            return;
        }
        if self.ui.button(cx, ids!(home_test_video)).clicked(actions) {
            self.test_video(cx);
            return;
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    /// Initialize the 5 tab buttons with their proper emoji + label.
    /// Called from handle_startup after the widget tree is built.
    fn init_tabs(&mut self, cx: &mut Cx) {
        let tabs: &[(&[LiveId], &str, &str)] = &[
            (ids!(tab_home),     "◐", "广场"),
            (ids!(tab_compose),  "✎", "录入"),
            (ids!(tab_studio),   "◇", "工坊"),
            (ids!(tab_plan),     "▤", "计划"),
            (ids!(tab_settings), "⚙", "设置"),
        ];
        for (id, emoji, label) in tabs.iter() {
            self.ui.button(cx, id).set_text(cx, &format!("{}\n{}", emoji, label));
        }
    }

    /// Test the image API (C6). Reads the API key from APP_STATE, calls
    /// Agnes 2.5 Flash `generate()`, and surfaces the result (or error)
    /// in the status label.
    fn test_image(&mut self, cx: &mut Cx) {
        use octostudio_ai::ImageClient;
        let key = state().api_key.clone();
        let key = match key {
            Some(k) if !k.is_empty() => k,
            _ => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 未设置 API key — 打开 设置 输入");
                return;
            }
        };
        let client = ImageClient::new(key);
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, "AI 配图中…(5–30 秒)");
        self.ui.redraw(cx);
        match client.generate(
            "a luminous floating city above a misty canyon at sunrise, cinematic realism",
            "1K",
            "16:9",
        ) {
            Ok(url) => {
                let msg = format!("✓ 配图 URL: {}", &url[..url.len().min(80)]);
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
            Err(e) => {
                let msg = format!("✗ 配图失败: {e}");
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
        }
        self.ui.redraw(cx);
    }

    /// Test the video API (C7). Reads the API key, calls
    /// `Agnes 2.5 create_text` (returns a video_id), then polls every
    /// 2s up to a 30s budget. Surfaces the result in the status label.
    fn test_video(&mut self, cx: &mut Cx) {
        use std::time::Duration;
        use octostudio_ai::video::VideoStatus;
        use octostudio_ai::VideoClient;
        let key = state().api_key.clone();
        let key = match key {
            Some(k) if !k.is_empty() => k,
            _ => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 未设置 API key — 打开 设置 输入");
                return;
            }
        };
        let client = VideoClient::new(key);
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, "🎬 视频提交中…");
        self.ui.redraw(cx);
        match client.create_and_wait(
            "a slow cinematic pan across a misty mountain ridge at sunrise",
            "4",
            "720P",
            "16:9",
            Duration::from_secs(2),
            Duration::from_secs(30),
        ) {
            Ok(VideoStatus::Completed(url)) => {
                let msg = format!("✓ 视频 URL: {}", &url[..url.len().min(80)]);
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
            Ok(VideoStatus::Failed(e)) => {
                let msg = format!("✗ 视频生成失败: {e}");
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
            Ok(other) => {
                let msg = format!("… 视频仍在: {:?}", other);
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
            Err(e) => {
                let msg = format!("✗ 视频调用失败: {e}");
                self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
            }
        }
        self.ui.redraw(cx);
    }
}
