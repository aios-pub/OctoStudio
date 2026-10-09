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
        spacing: 8
        draw_bg.color: #xF6F6F8

        home_title := Label{
            text: "广场"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 22
        }
        home_subtitle := Label{
            text: "C11 — 搜索 + 3 种排序 + 多选 + 真实 works 列表"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }

        // 搜索行
        View{
            width: Fill height: Fit
            flow: Right spacing: 6
            home_search := TextInput{
                width: Fill height: 32
                empty_text: "搜索标题 / 原文关键词..."
                draw_bg +: { color: #xFFFFFF color_focus: #xF6F6F8 border_radius: 8.0 }
            }
            home_search_clear := ButtonFlat{
                text: "清除"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xF6F6F8 border_radius: 16.0 }
                draw_text +: { color: #x1C1C1E }
            }
            home_select_toggle := ButtonFlat{
                text: "多选"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xF6F6F8 border_radius: 16.0 }
                draw_text +: { color: #x1C1C1E }
            }
            home_delete_selected := ButtonFlat{
                text: "删除所选"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xB3541E color_hover: #xD66030 border_radius: 16.0 }
                draw_text +: { color: #xFFFFFF }
            }
        }

        // 排序 chip 行
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            home_sort_desc := ButtonFlat{ text: "⇅ 最新" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xFF6B35 border_radius: 13.0 } draw_text +: { color: #xFFFFFF } }
            home_sort_asc := ButtonFlat{ text: "⇅ 最早" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF6F6F8 border_radius: 13.0 } draw_text +: { color: #x1C1C1E } }
            home_sort_kind := ButtonFlat{ text: "⇅ 按场景" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF6F6F8 border_radius: 13.0 } draw_text +: { color: #x1C1C1E } }
        }

        // 多选条(只在 select_mode 显示)
        home_select_bar := View{
            width: Fill height: Fit
            flow: Right spacing: 8
            visible: false
            home_select_label := Label{
                text: "已选 0 项"
                draw_text.color: #xFF6B35
                draw_text.text_style.font_size: 12
            }
        }

        // works 列表
        home_works_list := View{
            width: Fill height: Fill
            flow: Down spacing: 6
            home_works_empty := Label{
                text: "(无作品)"
                draw_text.color: #x8E8E93
                draw_text.text_style.font_size: 12
            }
        }

        // 测试按钮区(沿用 C6-C8)
        View{
            width: Fill height: Fit
            flow: Right spacing: 6
            home_test_image := ButtonFlat{
                text: "🎨 测试 AI 配图(首作品)"
                height: 36 padding: Inset{left: 14, right: 14}
                draw_bg +: { color: #xFF6B35 color_hover: #xFF8866 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
            home_test_video := ButtonFlat{
                text: "🎬 测试 AI 视频(首作品)"
                height: 36 padding: Inset{left: 14, right: 14}
                draw_bg +: { color: #x4A6FA5 color_hover: #x6A8FC5 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
            home_test_export := ButtonFlat{
                text: "📄 测试导出"
                height: 36 padding: Inset{left: 14, right: 14}
                draw_bg +: { color: #x2E7D5B color_hover: #x4E9D7B border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
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
            text: "C11 — 3 输入卡 + 「生成」调 TextClient::ask_scenario"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }

        // 场景 chip 行(简易版)
        View{
            width: Fill height: Fit
            flow: Down spacing: 6

            Label{
                text: "场景"
                draw_text.color: #x1C1C1E
                draw_text.text_style.font_size: 12
            }
            compose_scenarios := View{
                width: Fill height: Fit
                flow: Right spacing: 4
                compose_s1 := ButtonFlat{ text: "📝 原文二创" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xFF6B35 border_radius: 14.0 } draw_text +: { color: #xFFFFFF } }
                compose_s2 := ButtonFlat{ text: "🎬 视频分镜" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF6F6F8 border_radius: 14.0 } draw_text +: { color: #x1C1C1E } }
                compose_s3 := ButtonFlat{ text: "📷 图文" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF6F6F8 border_radius: 14.0 } draw_text +: { color: #x1C1C1E } }
                compose_s4 := ButtonFlat{ text: "🏷️ 标题" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF6F6F8 border_radius: 14.0 } draw_text +: { color: #x1C1C1E } }
                compose_s5 := ButtonFlat{ text: "✨ 金句" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF6F6F8 border_radius: 14.0 } draw_text +: { color: #x1C1C1E } }
            }
        }

        // 主题 chip 行
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            compose_theme1 := ButtonFlat{ text: "☀ 浅色" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF6F6F8 border_radius: 13.0 } draw_text +: { color: #x1C1C1E } }
            compose_theme2 := ButtonFlat{ text: "🌙 情感" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF6F6F8 border_radius: 13.0 } draw_text +: { color: #x1C1C1E } }
            compose_theme3 := ButtonFlat{ text: "🔬 科普" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF6F6F8 border_radius: 13.0 } draw_text +: { color: #x1C1C1E } }
        }

        // 一句话意图
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{ text: "一句话意图" draw_text.color: #x8E8E93 draw_text.text_style.font_size: 11 }
            compose_prompt := TextInput{
                width: Fill height: 36
                empty_text: "如:夏日海边慢生活,温暖散文风"
                draw_bg +: { color: #xFFFFFF color_focus: #xF6F6F8 border_radius: 8.0 }
            }
        }

        // 文字稿 / 原文
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{ text: "文字稿 / 原文" draw_text.color: #x8E8E93 draw_text.text_style.font_size: 11 }
            compose_source := TextInput{
                width: Fill height: 120
                empty_text: "粘贴视频文字稿 / 原文 / 详细描述..."
                draw_bg +: { color: #xFFFFFF color_focus: #xF6F6F8 border_radius: 8.0 }
            }
        }

        // 「生成」按钮
        View{
            width: Fill height: Fit
            flow: Right spacing: 8
            compose_generate := ButtonFlat{
                text: "✨ 生成(AI 或 demo)"
                height: 40 padding: Inset{left: 18, right: 18}
                draw_bg +: { color: #xFF6B35 color_hover: #xFF8866 border_radius: 20.0 }
                draw_text +: { color: #xFFFFFF }
            }
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
            text: "C11 — 计划条目 + 每条「AI 配图」chip(占位;真实生成在 C12)"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
        plan_summary_label := Label{
            text: "(无计划)"
            draw_text.color: #x1C1C1E
            draw_text.text_style.font_size: 14
        }
        plan_list := View{
            width: Fill height: Fit
            flow: Down spacing: 8
            plan_list_placeholder := Label{
                text: "在「录入」屏选场景 + 点「生成」后,条目会出现在这里。"
                draw_text.color: #x8E8E93
                draw_text.text_style.font_size: 12
            }
        }
        // 「AI 配图」按钮 — 真实调用 ImageClient::generate
        plan_gen_image := ButtonFlat{
            text: "🎨 AI 配图(按当前 plan_items 第一个 image_prompt)"
            height: 36 padding: Inset{left: 14, right: 14}
            draw_bg +: { color: #xFF6B35 color_hover: #xFF8866 border_radius: 18.0 }
            draw_text +: { color: #xFFFFFF }
        }
        plan_image_url_label := Label{
            text: "(尚未配图)"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 11
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
            text: "Agnes 3.0 Flash / 2.5 Image / 2.5 Video · OpenAI 兼容"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }

        // API key card
        View{
            width: Fill height: Fit
            flow: Down padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xE5E5EA

            Label{
                text: "🔑 API key (apihub.agnes-ai.com)"
                draw_text.color: #x1C1C1E
                draw_text.text_style.font_size: 14
            }
            settings_key_input := TextInput{
                width: Fill height: 32
                empty_text: "sk-..."
                draw_bg +: {
                    color: #xF6F6F8
                    color_focus: #xFFFFFF
                    border_radius: 8.0
                }
            }
            View{
                width: Fill height: Fit
                flow: Right spacing: 8
                settings_key_save := ButtonFlat{
                    text: "保存"
                    height: 32 padding: Inset{left: 14, right: 14}
                    draw_bg +: {
                        color: #xFF6B35
                        color_hover: #xFF8866
                        border_radius: 16.0
                    }
                    draw_text +: { color: #xFFFFFF }
                }
                settings_key_test := ButtonFlat{
                    text: "测试连接"
                    height: 32 padding: Inset{left: 14, right: 14}
                    draw_bg +: {
                        color: #x2E7D5B
                        color_hover: #x4E9D7B
                        border_radius: 16.0
                    }
                    draw_text +: { color: #xFFFFFF }
                }
            }
        }

        // Theme card
        View{
            width: Fill height: Fit
            flow: Down padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xE5E5EA

            Label{
                text: "🎨 主题"
                draw_text.color: #x1C1C1E
                draw_text.text_style.font_size: 14
            }
            settings_theme_chip := ButtonFlat{
                text: "☀ 浅色(v0.6 默认)"
                height: 32 padding: Inset{left: 14, right: 14}
                draw_bg +: {
                    color: #xF6F6F8
                    color_hover: #xE5E5EA
                    border_radius: 16.0
                }
                draw_text +: { color: #x1C1C1E }
            }
            Label{
                text: "深色 / 系统:留 v0.6.1"
                draw_text.color: #x8E8E93
                draw_text.text_style.font_size: 11
            }
        }

        // Usage card
        View{
            width: Fill height: Fit
            flow: Down padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xE5E5EA

            Label{
                text: "📊 配额 (今日)"
                draw_text.color: #x1C1C1E
                draw_text.text_style.font_size: 14
            }
            settings_usage_label := Label{
                text: "— 调用 / — tokens(usage.json 累计)"
                draw_text.color: #x8E8E93
                draw_text.text_style.font_size: 12
            }
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
        if self.ui.button(cx, ids!(home_test_export)).clicked(actions) {
            self.test_export(cx);
            return;
        }
        if self.ui.button(cx, ids!(settings_key_save)).clicked(actions) {
            self.save_api_key(cx);
            return;
        }
        if self.ui.button(cx, ids!(settings_key_test)).clicked(actions) {
            self.test_connection(cx);
            return;
        }
        // Compose scenario chips (5 presets)
        if self.ui.button(cx, ids!(compose_s1)).clicked(actions) {
            self.switch_scenario(cx, ""); // 原文二创
            return;
        }
        if self.ui.button(cx, ids!(compose_s2)).clicked(actions) {
            self.switch_scenario(cx, "video");
            return;
        }
        if self.ui.button(cx, ids!(compose_s3)).clicked(actions) {
            self.switch_scenario(cx, "image");
            return;
        }
        if self.ui.button(cx, ids!(compose_s4)).clicked(actions) {
            self.switch_scenario(cx, "titles");
            return;
        }
        if self.ui.button(cx, ids!(compose_s5)).clicked(actions) {
            self.switch_scenario(cx, "quotes");
            return;
        }
        if self.ui.button(cx, ids!(compose_generate)).clicked(actions) {
            self.ask_scenario_handler(cx);
            return;
        }
        if self.ui.button(cx, ids!(plan_gen_image)).clicked(actions) {
            self.plan_generate_image_handler(cx);
            return;
        }
        // Home — sort chips
        if self.ui.button(cx, ids!(home_sort_desc)).clicked(actions) {
            self.home_sort(cx, "updated_desc");
            return;
        }
        if self.ui.button(cx, ids!(home_sort_asc)).clicked(actions) {
            self.home_sort(cx, "updated_asc");
            return;
        }
        if self.ui.button(cx, ids!(home_sort_kind)).clicked(actions) {
            self.home_sort(cx, "kind");
            return;
        }
        if self.ui.button(cx, ids!(home_search_clear)).clicked(actions) {
            self.ui.text_input(cx, ids!(home_search)).set_text(cx, "");
            state().search_query = String::new();
            self.refresh_works_list(cx);
            return;
        }
        if self.ui.button(cx, ids!(home_select_toggle)).clicked(actions) {
            self.toggle_select_mode(cx);
            return;
        }
        if self.ui.button(cx, ids!(home_delete_selected)).clicked(actions) {
            self.batch_delete_selected(cx);
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
        // Hydrate Settings fields from current config
        let api_key = state().api_key.clone().unwrap_or_default();
        if !api_key.is_empty() {
            let placeholder = format!("sk-…(当前:{:.8})", api_key);
            self.ui.text_input(cx, ids!(settings_key_input))
                .set_text(cx, "");
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, &format!("已加载 API key ({})", &placeholder));
        }
        self.refresh_usage_label(cx);
    }

    /// Reload the usage.json roll-up into the Settings usage label.
    fn refresh_usage_label(&mut self, cx: &mut Cx) {
        use octostudio_storage::usage::load_usage;
        match load_usage() {
            Ok(log) => {
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                let row = log.days.iter().find(|d| d.date == today);
                let (calls, ptok, ctok) = row
                    .map(|d| (d.calls, d.prompt_tokens, d.completion_tokens))
                    .unwrap_or((0, 0, 0));
                let total_calls: u32 = log.days.iter().map(|d| d.calls).sum();
                let total_tok: u64 = log.days.iter().map(|d| d.prompt_tokens + d.completion_tokens).sum();
                let msg = format!(
                    "今日 {} 调用 / {} tokens · 累计 {} 调用 / {} tokens",
                    calls, ptok + ctok, total_calls, total_tok
                );
                self.ui.label(cx, ids!(settings_usage_label)).set_text(cx, &msg);
            }
            Err(_) => {
                self.ui.label(cx, ids!(settings_usage_label))
                    .set_text(cx, "(usage.json 暂无数据)");
            }
        }
    }

    /// Save the current API key from the Settings TextInput into config.json
    /// and APP_STATE. Surfaces success/failure in the status label.
    fn save_api_key(&mut self, cx: &mut Cx) {
        use octostudio_storage::config::{save_config, Config};
        let raw = self.ui.text_input(cx, ids!(settings_key_input)).text();
        let key = raw.trim().to_string();
        if key.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ API key 为空");
            return;
        }
        let cfg = Config {
            api_key: Some(key.clone()),
            ..Config::default()
        };
        match save_config(&cfg) {
            Ok(_) => {
                state().api_key = Some(key.clone());
                let masked = if key.len() > 8 {
                    format!("{}…{}", &key[..4], &key[key.len()-4..])
                } else { "***".to_string() };
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ API key 已保存 ({})", masked));
            }
            Err(e) => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ 保存失败: {e}"));
            }
        }
        self.ui.redraw(cx);
    }

    /// Call Agnes 3.0 Flash with a 1-token ping to verify connectivity.
    /// Surfaces result in the status label.
    fn test_connection(&mut self, cx: &mut Cx) {
        use octostudio_ai::TextClient;
        let key = state().api_key.clone();
        let key = match key {
            Some(k) if !k.is_empty() => k,
            _ => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 先保存 API key");
                return;
            }
        };
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, "🔄 测试连接中…(Agnes 3.0 Flash)");
        self.ui.redraw(cx);
        let client = TextClient::new(key);
        match client.chat("你只回答 OK", "ping") {
            Ok(reply) => {
                let preview = if reply.len() > 40 { &reply[..40] } else { &reply };
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ 连接成功 — reply: {}", preview));
            }
            Err(e) => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ 连接失败: {e}"));
            }
        }
        self.ui.redraw(cx);
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

    /// Test the export renderers (C8). Renders the first demo work in
    /// 6 formats and surfaces a short status line saying which one
    /// produced the most output.
    fn test_export(&mut self, cx: &mut Cx) {
        use octostudio_core::ExportFormat;
        use octostudio_export::render_plan_as_text;
        let works = state().works.clone();
        if works.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 无作品可导出");
            return;
        }
        let work = works[0].clone();
        let formats = [
            ExportFormat::Markdown,
            ExportFormat::Wechat,
            ExportFormat::Notion,
            ExportFormat::Srt,
            ExportFormat::Pack,
            ExportFormat::Marp,
        ];
        let mut sizes: Vec<(ExportFormat, usize)> = formats
            .iter()
            .map(|f| {
                let prev = work.plan_format;
                let mut w = work.clone();
                w.plan_format = *f;
                let s = render_plan_as_text(&w);
                w.plan_format = prev; // restore (work is consumed otherwise)
                (*f, s.len())
            })
            .collect();
        sizes.sort_by(|a, b| b.1.cmp(&a.1));
        let summary: Vec<String> = sizes
            .iter()
            .map(|(f, n)| format!("{:?}: {}B", f, n))
            .collect();
        let msg = format!("✓ 导出渲染: {}", summary.join(" · "));
        self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
        self.ui.redraw(cx);
    }

    /// Compose — switch scenario by id (5 presets in the chip row).
    /// Tints the selected chip accent and resets the inputs' plan_kind
    /// in APP_STATE. (Theme toggle deferred — v0.6.1.)
    fn switch_scenario(&mut self, cx: &mut Cx, kind_id: &str) {
        use octostudio_core::PlanKind;
        let kind = PlanKind::from_id(kind_id);
        state().plan_kind = kind;
        let label = format!("✓ 切到场景: {} ({})", kind.name(), kind.short());
        self.ui.label(cx, ids!(status_label)).set_text(cx, &label);
        self.ui.redraw(cx);
    }

    /// Compose — 「生成」button handler (C11).
    /// Reads intent / source from the TextInputs, calls
    /// `TextClient::ask_scenario(kind, intent, source, None, None)`.
    /// On success: fills `current_plan_*` + navigates to Plan.
    /// On failure (no key or network error): falls back to
    /// `octostudio_demo::load_demo_plan(kind)` so the user always sees
    /// a populated plan.
    fn ask_scenario_handler(&mut self, cx: &mut Cx) {
        use octostudio_ai::TextClient;
        use octostudio_core::PlanKind;
        use octostudio_demo::load_demo_plan;
        let kind = state().plan_kind;
        let intent = self.ui.text_input(cx, ids!(compose_prompt)).text();
        let source = self.ui.text_input(cx, ids!(compose_source)).text();
        let intent = intent.trim().to_string();
        let source = source.trim().to_string();
        if intent.is_empty() && source.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 至少填一个意图或原文");
            return;
        }
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("🤖 生成中…(kind={})", kind.short()));
        self.ui.redraw(cx);

        // Try real API first if key present (C12 will absorb the
        // returned JSON; for C11 we just probe success/failure to
        // distinguish the status message).
        let has_key = state()
            .api_key
            .as_ref()
            .map(|k| !k.is_empty())
            .unwrap_or(false);
        if has_key {
            if let Some(k) = state().api_key.clone() {
                let client = TextClient::new(k);
                match client.ask_scenario(kind, &intent, &source, None, None) {
                    Ok(_) => {
                        // API worked — C11 ignores the parsed plan and
                        // still shows the demo, but logs the success.
                    }
                    Err(e) => {
                        self.ui.label(cx, ids!(status_label))
                            .set_text(cx, &format!("⚠ Agnes 调用失败(已回退 demo): {e}"));
                    }
                }
            }
        }

        // For C11 we always go via load_demo_plan (real Agnes result
        // will be absorbed in C12).
        let (title, summary, items, comp) = load_demo_plan(kind);
        {
            let mut s = state();
            s.current_prompt = intent.clone();
            s.current_source = source.clone();
            s.current_plan_title = title.clone();
            s.current_plan_summary = summary.clone();
            s.current_plan_items = items.clone();
            s.current_composition = comp;
        }
        self.ui.label(cx, ids!(plan_summary_label))
            .set_text(cx, &format!("{} ({} 条)", title, items.len()));
        let msg = if has_key {
            format!("✓ 生成完成(API 已调, 显示 demo) — {} 条", items.len())
        } else {
            format!("✓ 生成完成(demo,无 API key) — {} 条", items.len())
        };
        self.ui.label(cx, ids!(status_label)).set_text(cx, &msg);
        self.goto(cx, Screen::Plan);
    }

    /// Plan — 「AI 配图」button handler.
    /// Reads the first plan_item's `image_prompt` (or `video_prompt` /
    /// `body` for xhs), calls `ImageClient::generate(prompt, "1K", "16:9")`,
    /// surfaces the URL in `plan_image_url_label`.
    fn plan_generate_image_handler(&mut self, cx: &mut Cx) {
        use octostudio_ai::ImageClient;
        let items = state().current_plan_items.clone();
        if items.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 无计划条目 — 先在「录入」生成");
            return;
        }
        let prompt = items
            .iter()
            .map(|i| {
                let p = i.get("image_prompt");
                if !p.is_empty() { return p.to_string(); }
                let v = i.get("video_prompt");
                if !v.is_empty() { return v.to_string(); }
                let b = i.get("body");
                if !b.is_empty() { return b.to_string(); }
                String::new()
            })
            .find(|s| !s.is_empty())
            .unwrap_or_default();
        if prompt.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 条目无 image_prompt");
            return;
        }
        let key = state().api_key.clone();
        let key = match key.filter(|k| !k.is_empty()) {
            Some(k) => k,
            None => {
                self.ui.label(cx, ids!(plan_image_url_label))
                    .set_text(cx, "⚠ 无 API key — 打开 设置 填入");
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 无 API key");
                return;
            }
        };
        self.ui.label(cx, ids!(plan_image_url_label))
            .set_text(cx, "🎨 配图中…(5-30s)");
        self.ui.redraw(cx);
        let client = ImageClient::new(key);
        match client.generate(&prompt, "1K", "16:9") {
            Ok(url) => {
                self.ui.label(cx, ids!(plan_image_url_label))
                    .set_text(cx, &format!("✓ URL: {}", &url[..url.len().min(80)]));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "✓ 配图完成");
            }
            Err(e) => {
                self.ui.label(cx, ids!(plan_image_url_label))
                    .set_text(cx, &format!("✗ 配图失败: {e}"));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ 配图失败: {e}"));
            }
        }
        self.ui.redraw(cx);
    }

    /// Home — sort works by mode. Tints the selected chip accent.
    fn home_sort(&mut self, cx: &mut Cx, mode: &str) {
        use octostudio_core::SortMode;
        state().sort_mode = SortMode::from_id(mode);
        self.refresh_works_list(cx);
        let label = match mode {
            "updated_desc" => "⇅ 最新",
            "updated_asc" => "⇅ 最早",
            "kind" => "⇅ 按场景",
            _ => mode,
        };
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 排序:{label}"));
    }

    /// Home — re-render works list label with current filter + sort.
    /// (C11 simplification: text-only; widget row rendering is a follow-up.)
    fn refresh_works_list(&mut self, cx: &mut Cx) {
        use octostudio_core::{PlanKind, SortMode};
        let q = state().search_query.to_lowercase();
        let sort_mode = state().sort_mode;
        let mut works = state().works.clone();
        if !q.is_empty() {
            works.retain(|w| {
                w.title.to_lowercase().contains(&q)
                    || w.source.to_lowercase().contains(&q)
                    || w.kind_label().to_lowercase().contains(&q)
            });
        }
        match sort_mode {
            SortMode::UpdatedDesc => works.sort_by(|a, b| b.updated.cmp(&a.updated)),
            SortMode::UpdatedAsc => works.sort_by(|a, b| a.updated.cmp(&b.updated)),
            SortMode::ByKind => {
                works.sort_by(|a, b| {
                    a.plan_kind.unwrap_or(PlanKind::None).short()
                        .cmp(&b.plan_kind.unwrap_or(PlanKind::None).short())
                })
            }
        }
        let total = state().works.len();
        let shown = works.len();
        let summary = if q.is_empty() {
            format!("C11 — 共 {} / {} 件 · {}", shown, total, sort_mode_label(sort_mode))
        } else {
            format!("C11 — \"{}\" 匹配 {} / {} 件", q, shown, total)
        };
        let preview: String = works.iter().take(5).map(|w| {
            format!("  • {} [{}]", w.title, w.kind_label())
        }).collect::<Vec<_>>().join("\n");
        let body = if preview.is_empty() { summary.clone() } else { format!("{summary}\n{preview}") };
        self.ui.label(cx, ids!(home_works_empty)).set_text(cx, &body);
        self.ui.label(cx, ids!(home_subtitle)).set_text(cx, &summary);
        self.ui.redraw(cx);
    }

    /// Home — toggle multi-select mode + show/hide the select bar.
    fn toggle_select_mode(&mut self, cx: &mut Cx) {
        let entering = !state().select_mode;
        state().select_mode = entering;
        if !entering {
            state().selected_ids.clear();
        }
        self.ui.view(cx, ids!(home_select_bar))
            .set_visible(cx, entering);
        self.ui.button(cx, ids!(home_select_toggle))
            .set_text(cx, if entering { "完成" } else { "多选" });
        let count = state().selected_ids.len();
        let label = if entering { format!("已选 {count} 项") } else { "多选".to_string() };
        self.ui.label(cx, ids!(home_select_label)).set_text(cx, &label);
        let status = if entering {
            "✓ 多选模式(条目行 toggle 选中留 C12)"
        } else {
            "✓ 退出多选"
        };
        self.ui.label(cx, ids!(status_label)).set_text(cx, status);
        self.ui.redraw(cx);
    }

    /// Home — delete all works in the selected set, persist, refresh.
    fn batch_delete_selected(&mut self, cx: &mut Cx) {
        if !state().select_mode {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 先进入多选模式");
            return;
        }
        let n = state().selected_ids.len();
        if n == 0 {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 没选任何作品");
            return;
        }
        let ids: Vec<String> = state().selected_ids.iter().cloned().collect();
        {
            let mut s = state();
            s.works.retain(|w| !ids.contains(&w.id));
            s.selected_ids.clear();
        }
        let works = state().works.clone();
        if let Err(e) = octostudio_storage::works::save_works(&works) {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, &format!("✗ 删除失败: {e}"));
            return;
        }
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 已删 {n} 件,剩 {} 件", works.len()));
        self.refresh_works_list(cx);
    }
}

fn sort_mode_label(m: octostudio_core::SortMode) -> &'static str {
    use octostudio_core::SortMode::*;
    match m {
        UpdatedDesc => "最新优先",
        UpdatedAsc => "最早优先",
        ByKind => "按场景分组",
    }
}
