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

pub static APP_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

pub fn state() -> MutexGuard<'static, AppState> {
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
        draw_bg.color: #xFFFFFF

        // 屏头(sticky)— 白底标题区
        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            home_title := Label{
                text: "广场"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            home_subtitle := Label{
                text: "本地作品库 · 搜索 · 排序 · 批量管理"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
        }

        // 内容区 — 灰底 + 白卡,可滚动
        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 10
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // 搜索行
        View{
            width: Fill height: Fit
            flow: Right spacing: 6
            home_search := TextInput{
                width: Fill height: 32
                empty_text: "搜索标题 / 原文关键词..."
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 8.0 }
            }
            home_search_clear := ButtonFlat{
                text: "清除"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xF2F3F5 border_radius: 16.0 }
                draw_text +: { color: #x1F2329 }
            }
            home_select_toggle := ButtonFlat{
                text: "多选"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xF2F3F5 border_radius: 16.0 }
                draw_text +: { color: #x1F2329 }
            }
            home_delete_selected := ButtonFlat{
                text: "删除所选"
                height: 32 padding: Inset{left: 12, right: 12}
                draw_bg +: { color: #xF54A45 color_hover: #xFF6157 border_radius: 16.0 }
                draw_text +: { color: #xFFFFFF }
            }
        }

        // 排序 chip 行
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            home_sort_desc := ButtonFlat{ text: "⇅ 最新" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #x3370FF border_radius: 13.0 } draw_text +: { color: #xFFFFFF } }
            home_sort_asc := ButtonFlat{ text: "⇅ 最早" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF2F3F5 border_radius: 13.0 } draw_text +: { color: #x1F2329 } }
            home_sort_kind := ButtonFlat{ text: "⇅ 按场景" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF2F3F5 border_radius: 13.0 } draw_text +: { color: #x1F2329 } }
        }

        // 多选条(只在 select_mode 显示)
        home_select_bar := View{
            width: Fill height: Fit
            flow: Right spacing: 8
            visible: false
            home_select_label := Label{
                text: "已选 0 项"
                draw_text.color: #x3370FF
                draw_text.text_style.font_size: 12
            }
        }

        // works 列表
        home_works_list := View{
            width: Fill height: Fit
            flow: Down spacing: 6
            home_works_empty := Label{
                text: "(无作品)"
                draw_text.color: #x646A73
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
                draw_bg +: { color: #x3370FF color_hover: #x5B8CFF border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
            home_test_video := ButtonFlat{
                text: "🎬 测试 AI 视频(首作品)"
                height: 36 padding: Inset{left: 14, right: 14}
                draw_bg +: { color: #x245BDB color_hover: #x4A79F0 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
            home_test_export := ButtonFlat{
                text: "📄 测试导出"
                height: 36 padding: Inset{left: 14, right: 14}
                draw_bg +: { color: #x34C724 color_hover: #x54D644 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF }
            }
        }
        }
    }

    let ComposeView = View{
        width: Fill height: Fill
        flow: Down
        draw_bg.color: #xFFFFFF

        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            compose_title := Label{
                text: "开始一段创作"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            compose_subtitle := Label{
                text: "一句话意图 → 结构化创作(AI 或演示数据)"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
        }

        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 12
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // 场景 chip 行(简易版)
        View{
            width: Fill height: Fit
            flow: Down spacing: 6

            Label{
                text: "场景"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 12
            }
            compose_scenarios := View{
                width: Fill height: Fit
                flow: Right spacing: 4
                compose_s1 := ButtonFlat{ text: "📝 原文二创" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #x3370FF border_radius: 14.0 } draw_text +: { color: #xFFFFFF } }
                compose_s2 := ButtonFlat{ text: "🎬 视频分镜" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
                compose_s3 := ButtonFlat{ text: "📷 图文" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
                compose_s4 := ButtonFlat{ text: "🏷️ 标题" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
                compose_s5 := ButtonFlat{ text: "✨ 金句" height: 28 padding: Inset{left: 10, right: 10}
                    draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            }
        }

        // 主题 chip 行
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            compose_theme1 := ButtonFlat{ text: "☀ 浅色" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF2F3F5 border_radius: 13.0 } draw_text +: { color: #x1F2329 } }
            compose_theme2 := ButtonFlat{ text: "🌙 情感" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF2F3F5 border_radius: 13.0 } draw_text +: { color: #x1F2329 } }
            compose_theme3 := ButtonFlat{ text: "🔬 科普" height: 26 padding: Inset{left: 10, right: 10}
                draw_bg +: { color: #xF2F3F5 border_radius: 13.0 } draw_text +: { color: #x1F2329 } }
        }

        // 一句话意图
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{ text: "一句话意图" draw_text.color: #x646A73 draw_text.text_style.font_size: 11 }
            compose_prompt := TextInput{
                width: Fill height: 36
                empty_text: "如:夏日海边慢生活,温暖散文风"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 8.0 }
            }
        }

        // 文字稿 / 原文
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{ text: "文字稿 / 原文" draw_text.color: #x646A73 draw_text.text_style.font_size: 11 }
            compose_source := TextInput{
                width: Fill height: 120
                empty_text: "粘贴视频文字稿 / 原文 / 详细描述..."
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 8.0 }
            }
        }

        // 「生成」按钮
        View{
            width: Fill height: Fit
            flow: Right spacing: 8
            compose_generate := ButtonFlat{
                text: "✨ 生成(AI 或 demo)"
                height: 40 padding: Inset{left: 18, right: 18}
                draw_bg +: { color: #x3370FF color_hover: #x5B8CFF border_radius: 20.0 }
                draw_text +: { color: #xFFFFFF }
            }
        }
        }
    }

    let StudioView = View{
        width: Fill height: Fill
        flow: Down
        draw_bg.color: #xFFFFFF

        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            studio_title := Label{
                text: "工坊"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            studio_subtitle := Label{
                text: "8 种风格二创 · 7 项 AI 助手 · 撤销 / 快照"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
        }

        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 10
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // 8 style chip 横排
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            studio_s1 := ButtonFlat{ text: "改写" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #x3370FF border_radius: 14.0 } draw_text +: { color: #xFFFFFF } }
            studio_s2 := ButtonFlat{ text: "翻译" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s3 := ButtonFlat{ text: "总结" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s4 := ButtonFlat{ text: "评论" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s5 := ButtonFlat{ text: "润色" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s6 := ButtonFlat{ text: "续写" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s7 := ButtonFlat{ text: "扩写" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            studio_s8 := ButtonFlat{ text: "小红书体" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
        }

        // AI 创作/重试/撤销/快照 + 标题输入
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{ text: "标题" draw_text.color: #x646A73 draw_text.text_style.font_size: 11 }
            studio_title_input := TextInput{
                width: Fill height: 36
                empty_text: "作品标题..."
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 8.0 }
            }
            View{ width: Fill height: Fit flow: Right spacing: 6
                studio_ai_write := ButtonFlat{ text: "✨ AI 创作(按当前 style 改写)"
                    height: 36 padding: Inset{left:14,right:14}
                    draw_bg +: { color: #x3370FF color_hover: #x5B8CFF border_radius: 18.0 }
                    draw_text +: { color: #xFFFFFF } }
                studio_ai_retry := ButtonFlat{ text: "🔄 重试"
                    height: 36 padding: Inset{left:14,right:14}
                    draw_bg +: { color: #xF2F3F5 border_radius: 18.0 }
                    draw_text +: { color: #x1F2329 } }
                studio_undo := ButtonFlat{ text: "↶ 撤销(0)"
                    height: 36 padding: Inset{left:14,right:14}
                    draw_bg +: { color: #xF2F3F5 border_radius: 18.0 }
                    draw_text +: { color: #x1F2329 } }
                studio_snapshot := ButtonFlat{ text: "📷 快照原文"
                    height: 36 padding: Inset{left:14,right:14}
                    draw_bg +: { color: #xF2F3F5 border_radius: 18.0 }
                    draw_text +: { color: #x1F2329 } }
            }
        }

        // 原文输入(从 current_source 同步)
        Label{ text: "原文 / 内容" draw_text.color: #x646A73 draw_text.text_style.font_size: 11 }
        studio_source_label := Label{ text: "(空 — 在「录入」填)"
            draw_text.color: #x1F2329 draw_text.text_style.font_size: 12 }

        // AI 助手 panel: 7 chips + result label
        View{
            width: Fill height: Fit
            flow: Down spacing: 6
            padding: Inset{top: 8, bottom: 8, left: 12, right: 12}
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xDEE0E3

            Label{ text: "🤖 AI 助手(7 项) — Agnes 3.0 Flash"
                draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
            View{ width: Fill height: Fit flow: Right spacing: 4
                studio_p1 := ButtonFlat{ text: "起标题" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p2 := ButtonFlat{ text: "关键词" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p3 := ButtonFlat{ text: "摘要" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p4 := ButtonFlat{ text: "风格迁移" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p5 := ButtonFlat{ text: "中英对照" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p6 := ButtonFlat{ text: "标题打分" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xE1EFFF border_radius: 12.0 } draw_text +: { color: #x3370FF } }
                studio_p7 := ButtonFlat{ text: "模型预算" height: 24 padding: Inset{left:8,right:8}
                    draw_bg +: { color: #xDEE0E3 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
            }
            studio_result_label := Label{
                text: "(点任意 AI 助手 chip,结果会显示在这里)"
                draw_text.color: #x646A73
                draw_text.text_style.font_size: 12
            }
        }
        }
    }

    let PlanView = View{
        width: Fill height: Fill
        flow: Down
        draw_bg.color: #xFFFFFF

        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            plan_title := Label{
                text: "计划"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            plan_subtitle := Label{
                text: "逐条编辑 · AI 配图 · AI 合成视频"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
            plan_summary_label := Label{
                text: "(无计划)"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 14
            }
        }

        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 10
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // C12 — render 8 hard-coded item cards. C16 wraps them in a
        // ScrollYView of fixed height 280 so 8 entries scroll when needed.
        plan_list := ScrollYView{
            width: Fill height: 280
            flow: Down spacing: 8
            plan_item0 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item0_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item0_idx := Label{ text: "第 1 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item0_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item0_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item0_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item0_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item0_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item1 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item1_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item1_idx := Label{ text: "第 2 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item1_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item1_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item1_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item1_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item1_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item2 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item2_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item2_idx := Label{ text: "第 3 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item2_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item2_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item2_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item2_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item2_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item3 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item3_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item3_idx := Label{ text: "第 4 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item3_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item3_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item3_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item3_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item3_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item4 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item4_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item4_idx := Label{ text: "第 5 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item4_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item4_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item4_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item4_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item4_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item5 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item5_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item5_idx := Label{ text: "第 6 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item5_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item5_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item5_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item5_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item5_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item6 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item6_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item6_idx := Label{ text: "第 7 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item6_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item6_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item6_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item6_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item6_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
            plan_item7 := View{ width: Fill height: Fit flow: Down padding: Inset{top:8,bottom:8,left:14,right:14} spacing: 6
                draw_bg.color: #xFFFFFF draw_bg.border_radius: 10.0
                plan_item7_header := View{ width: Fill height: Fit flow: Right spacing: 6
                    plan_item7_idx := Label{ text: "第 8 条" draw_text.color: #x1F2329 draw_text.text_style.font_size: 13 }
                    plan_item7_edit := ButtonFlat{ text: "✎ 编辑" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item7_up := ButtonFlat{ text: "↑ 上移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item7_down := ButtonFlat{ text: "↓ 下移" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF2F3F5 border_radius: 12.0 } draw_text +: { color: #x1F2329 } }
                    plan_item7_del := ButtonFlat{ text: "🗑 删除" height: 24 padding: Inset{left:8,right:8}
                        draw_bg +: { color: #xF54A45 border_radius: 12.0 } draw_text +: { color: #xFFFFFF } }
                }
                plan_item7_body := Label{ text: "(空)" draw_text.color: #x646A73 draw_text.text_style.font_size: 12 }
            }
        }

        // C16: 6 字段 TextInput editor card (max 6 fields per plan_kind for video)
        //     visible only when state().editing_k != -1
        //     字段 labels 由 item_fields(plan_kind) 决定 (image=3, video=6, ppt=5, ...)
        plan_editor_card := View{
            width: Fill height: Fit
            visible: false
            flow: Down spacing: 4
            padding: Inset{top: 8, bottom: 8, left: 10, right: 10}
            draw_bg.color: #xF0F5FF
            draw_bg.border_radius: 8.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #x3370FF

            plan_editor_title := Label{ text: "✎ 编辑条目 (按 Tab 切换字段)"
                draw_text.color: #x1F2329 draw_text.text_style.font_size: 12 }
            plan_field0 := TextInput{ width: Fill height: 28 empty_text: "field 0"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            plan_field1 := TextInput{ width: Fill height: 28 empty_text: "field 1"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            plan_field2 := TextInput{ width: Fill height: 28 empty_text: "field 2"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            plan_field3 := TextInput{ width: Fill height: 28 empty_text: "field 3"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            plan_field4 := TextInput{ width: Fill height: 28 empty_text: "field 4"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            plan_field5 := TextInput{ width: Fill height: 28 empty_text: "field 5"
                draw_bg +: { color: #xFFFFFF color_focus: #xF2F3F5 border_radius: 6.0 } }
            View{ width: Fill height: Fit flow: Right spacing: 6
                plan_editor_save := ButtonFlat{ text: "💾 写入"
                    height: 28 padding: Inset{left:12,right:12}
                    draw_bg +: { color: #x34C724 border_radius: 14.0 }
                    draw_text +: { color: #xFFFFFF } }
                plan_editor_cancel := ButtonFlat{ text: "✗ 取消"
                    height: 28 padding: Inset{left:12,right:12}
                    draw_bg +: { color: #xF2F3F5 border_radius: 14.0 }
                    draw_text +: { color: #x1F2329 } }
            }
        }

        // Action row
        View{
            width: Fill height: Fit
            flow: Right spacing: 8
            plan_item_add := ButtonFlat{ text: "➕ 新增一条"
                height: 36 padding: Inset{left:14,right:14}
                draw_bg +: { color: #x34C724 color_hover: #x54D644 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF } }
            plan_save_works := ButtonFlat{ text: "💾 保存到 works.json"
                height: 36 padding: Inset{left:14,right:14}
                draw_bg +: { color: #x3370FF color_hover: #x5B8CFF border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF } }
            plan_gen_image := ButtonFlat{ text: "🎨 AI 配图(首条 image_prompt)"
                height: 36 padding: Inset{left:14,right:14}
                draw_bg +: { color: #x245BDB color_hover: #x4A79F0 border_radius: 18.0 }
                draw_text +: { color: #xFFFFFF } }
            plan_to_export := ButtonFlat{ text: "📤 导出 →"
                height: 36 padding: Inset{left:14,right:14}
                draw_bg +: { color: #x646A73 border_radius: 18.0 }
                draw_text +: { color: #x1F2329 } }
        }
        plan_image_url_label := Label{ text: "(尚未配图)"
            draw_text.color: #x646A73 draw_text.text_style.font_size: 11 }
        }
    }

    let ExportView = View{
        width: Fill height: Fill
        flow: Down
        draw_bg.color: #xFFFFFF

        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            export_title := Label{
                text: "导出"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            export_subtitle := Label{
                text: "8 种格式一键复制:Markdown / 公众号 / Notion / SRT / 制作包 / Marp / Markmap"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
        }

        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 10
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // 8 格式 chip 横排
        View{
            width: Fill height: Fit
            flow: Right spacing: 4
            export_f1 := ButtonFlat{ text: "Markdown" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #x3370FF border_radius: 14.0 } draw_text +: { color: #xFFFFFF } }
            export_f2 := ButtonFlat{ text: "公众号" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f3 := ButtonFlat{ text: "Notion" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f4 := ButtonFlat{ text: "SRT 字幕" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f5 := ButtonFlat{ text: "视频制作包" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f6 := ButtonFlat{ text: "Marp 演示" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f7 := ButtonFlat{ text: "Markmap" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xF2F3F5 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
            export_f8 := ButtonFlat{ text: "全部预览" height: 28 padding: Inset{left:10,right:10}
                draw_bg +: { color: #xDEE0E3 border_radius: 14.0 } draw_text +: { color: #x1F2329 } }
        }

        // 适用格式提示
        export_hint_label := Label{
            text: "(选中一个格式 → 大 Card 实时渲染 → 复制)"
            draw_text.color: #x646A73
            draw_text.text_style.font_size: 11
        }

        // 大 Card 预览(白底圆角 10pt 1pt border)
        View{
            width: Fill height: 320
            flow: Down
            padding: Inset{top: 12, bottom: 12, left: 16, right: 16}
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xDEE0E3

            export_preview_label := Label{
                text: "(点击上方 8 个 chip 中的任一个,大 Card 会渲染该格式)"
                draw_text.color: #x646A73
                draw_text.text_style.font_size: 12
            }
        }

        // 复制 + 操作行
        View{
            width: Fill height: Fit
            flow: Right spacing: 8
            export_copy := ButtonFlat{ text: "📋 复制到剪贴板"
                height: 40 padding: Inset{left:18,right:18}
                draw_bg +: { color: #x3370FF color_hover: #x5B8CFF border_radius: 20.0 }
                draw_text +: { color: #xFFFFFF } }
            export_back := ButtonFlat{ text: "← 返回"
                height: 40 padding: Inset{left:18,right:18}
                draw_bg +: { color: #xF2F3F5 border_radius: 20.0 }
                draw_text +: { color: #x1F2329 } }
        }

        // 当前格式 + 字数状态
        export_status_label := Label{
            text: "(空)"
            draw_text.color: #x646A73
            draw_text.text_style.font_size: 11
        }
        }
    }

    let SettingsView = View{
        width: Fill height: Fill
        flow: Down
        draw_bg.color: #xFFFFFF

        View{
            width: Fill height: Fit
            flow: Down
            spacing: 2
            padding: Inset{top: 20, bottom: 10, left: 28, right: 28}
            settings_title := Label{
                text: "设置"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 20
            }
            settings_subtitle := Label{
                text: "Agnes 3.0 Flash / 2.5 Image / 2.5 Video · OpenAI 兼容"
                draw_text.color: #x8F959E
                draw_text.text_style.font_size: 12
            }
        }

        ScrollYView{
            width: Fill height: Fill
            flow: Down
            spacing: 12
            padding: Inset{top: 6, bottom: 20, left: 28, right: 28}
            draw_bg +: { color: #xF2F3F5 }

        // API key card
        View{
            width: Fill height: Fit
            flow: Down padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8
            draw_bg.color: #xFFFFFF
            draw_bg.border_radius: 10.0
            draw_bg.border_size: 1.0
            draw_bg.border_color: #xDEE0E3

            Label{
                text: "🔑 API key (apihub.agnes-ai.com)"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 14
            }
            settings_key_input := TextInput{
                width: Fill height: 32
                empty_text: "sk-..."
                draw_bg +: {
                    color: #xF2F3F5
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
                        color: #x3370FF
                        color_hover: #x5B8CFF
                        border_radius: 16.0
                    }
                    draw_text +: { color: #xFFFFFF }
                }
                settings_key_test := ButtonFlat{
                    text: "测试连接"
                    height: 32 padding: Inset{left: 14, right: 14}
                    draw_bg +: {
                        color: #x34C724
                        color_hover: #x54D644
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
            draw_bg.border_color: #xDEE0E3

            Label{
                text: "🎨 主题"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 14
            }
            settings_theme_chip := ButtonFlat{
                text: "☀ 浅色(v0.6 默认)"
                height: 32 padding: Inset{left: 14, right: 14}
                draw_bg +: {
                    color: #xF2F3F5
                    color_hover: #xDEE0E3
                    border_radius: 16.0
                }
                draw_text +: { color: #x1F2329 }
            }
            Label{
                text: "深色 / 系统:留 v0.6.1"
                draw_text.color: #x646A73
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
            draw_bg.border_color: #xDEE0E3

            Label{
                text: "📊 配额 (今日)"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 14
            }
            settings_usage_label := Label{
                text: "— 调用 / — tokens(usage.json 累计)"
                draw_text.color: #x646A73
                draw_text.text_style.font_size: 12
            }
        }
        }
    }

    // ---------- 飞书风左侧导航 ----------
    // 每个条目一对 off/on ButtonFlat:goto() 用 set_visible 切换选中态
    // (makepad 2.0 没有 apply_over,运行时换肤只有 set_visible/set_text)。

    let NavBtnOff = ButtonFlat{
        width: Fill height: 40
        text: "○"
        draw_bg +: {
            color: #xF7F8FA
            color_hover: #xE8EAED
            border_radius: 8.0
            border_size: 0.0
        }
        draw_text +: {
            color: #x1F2329
            text_style.font_size: 14
        }
    }

    let NavBtnOn = ButtonFlat{
        width: Fill height: 40
        text: "○"
        draw_bg +: {
            color: #xE1EFFF
            color_hover: #xD6E8FF
            border_radius: 8.0
            border_size: 0.0
        }
        draw_text +: {
            color: #x3370FF
            text_style.font_size: 14
        }
    }

    let Sidebar = View{
        width: 220 height: Fill
        flow: Down
        spacing: 2
        padding: Inset{top: 12, bottom: 14, left: 10, right: 10}
        draw_bg.color: #xF7F8FA
        draw_bg.border_size: 1.0
        draw_bg.border_color: #xDEE0E3

        // 品牌区
        View{
            width: Fill height: Fit
            flow: Right
            spacing: 8
            align: Align{y: 0.5}
            padding: Inset{top: 6, bottom: 14, left: 8, right: 8}
            brand_logo := Label{
                text: "◆"
                draw_text.color: #x3370FF
                draw_text.text_style.font_size: 18
            }
            brand_name := Label{
                text: "OctoStudio"
                draw_text.color: #x1F2329
                draw_text.text_style.font_size: 16
            }
        }

        nav_home_off := NavBtnOff{ text: "◐  广场" }
        nav_home_on := NavBtnOn{ text: "◐  广场" }
        nav_compose_off := NavBtnOff{ text: "✎  录入" }
        nav_compose_on := NavBtnOn{ text: "✎  录入" }
        nav_studio_off := NavBtnOff{ text: "◇  工坊" }
        nav_studio_on := NavBtnOn{ text: "◇  工坊" }
        nav_plan_off := NavBtnOff{ text: "▤  计划" }
        nav_plan_on := NavBtnOn{ text: "▤  计划" }
        nav_settings_off := NavBtnOff{ text: "⚙  设置" }
        nav_settings_on := NavBtnOn{ text: "⚙  设置" }

        // 弹性空白
        View{ width: Fill height: Fill }

        nav_status := Label{
            width: Fill
            text: "○ 未配置 API key"
            draw_text.color: #x8F959E
            draw_text.text_style.font_size: 11
        }
    }

    let AppFooter = View{
        width: Fill height: Fit
        flow: Right
        spacing: 8
        padding: Inset{top: 6, bottom: 6, left: 28, right: 28}
        align: Align{y: 0.5}
        draw_bg.color: #xFFFFFF
        draw_bg.border_size: 1.0
        draw_bg.border_color: #xE8EAED

        status_label := Label{
            width: Fill
            text: "就绪"
            draw_text.color: #x8F959E
            draw_text.text_style.font_size: 11
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
                        flow: Right
                        spacing: 0
                        draw_bg.color: #xFFFFFF

                        // 左:飞书风侧边导航
                        Sidebar{}

                        // 右:内容分屏(可滚动)+ 底部状态条
                        View{
                            width: Fill height: Fill
                            flow: Down
                            spacing: 0
                            draw_bg.color: #xFFFFFF

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
                        }
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
    /// Goto — flip the visible flag of each screen + sidebar nav state,
    /// then request a redraw.
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
        self.apply_nav(cx, mode);
        self.ui.redraw(cx);
    }

    /// Sidebar selected state: show the `on` (blue tint) variant of the
    /// active entry, the `off` variant of the rest. Export highlights none.
    fn apply_nav(&mut self, cx: &mut Cx, mode: Screen) {
        let entries = [
            (Screen::Home, ids!(nav_home_on), ids!(nav_home_off)),
            (Screen::Compose, ids!(nav_compose_on), ids!(nav_compose_off)),
            (Screen::Studio, ids!(nav_studio_on), ids!(nav_studio_off)),
            (Screen::Plan, ids!(nav_plan_on), ids!(nav_plan_off)),
            (Screen::Settings, ids!(nav_settings_on), ids!(nav_settings_off)),
        ];
        for (screen, on, off) in entries {
            self.ui.button(cx, on).set_visible(cx, mode == screen);
            self.ui.button(cx, off).set_visible(cx, mode != screen);
        }
    }

    /// Sidebar bottom status line — API key state.
    fn refresh_nav_status(&mut self, cx: &mut Cx) {
        let has_key = state()
            .api_key
            .as_deref()
            .map(|k| !k.trim().is_empty())
            .unwrap_or(false);
        let text = if has_key { "● API 已连接" } else { "○ 未配置 API key" };
        self.ui.label(cx, ids!(nav_status)).set_text(cx, text);
    }

    // -------- C12: Plan item actions (add / del / up / down / edit) --------

    /// Plan — dispatch one chip action (edit / up / down / del) on the
    /// plan_item at the given index (0..8). State is mutated, then the
    /// 8 body labels are refreshed. Persistence is decoupled — the user
    /// must click `save_plan_to_works` to write to disk.
    fn plan_item_action(&mut self, cx: &mut Cx, idx: u64, action: PlanItemAction) {
        match action {
            PlanItemAction::Edit => self.item_start_edit(cx, idx),
            PlanItemAction::Up => self.item_move(cx, idx, -1),
            PlanItemAction::Down => self.item_move(cx, idx, 1),
            PlanItemAction::Del => self.item_delete(cx, idx),
        }
    }

    /// Plan — 「➕ 新增一条」button. Push a `blank_item(kind)` onto
    /// `current_plan_items` (capped at 16; splash uses 16 as max).
    fn item_add(&mut self, cx: &mut Cx) {
        use octostudio_core::blank_item;
        let kind = state().plan_kind;
        let next = state().next_seq();
        {
            let mut s = state();
            if s.current_plan_items.len() < 16 {
                s.current_plan_items.push(blank_item(kind, next));
            } else {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 最多 16 条");
                return;
            }
        }
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 已新增 1 条,共 {} 条", state().current_plan_items.len()));
        self.refresh_plan_list(cx);
    }

    /// Plan — 「🗑 删除」chip on item `idx`.
    fn item_delete(&mut self, cx: &mut Cx, idx: u64) {
        let n = state().current_plan_items.len();
        if idx as usize >= n {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 越界");
            return;
        }
        {
            let mut s = state();
            s.current_plan_items.remove(idx as usize);
        }
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 已删第 {} 条,剩 {} 条", idx + 1, state().current_plan_items.len()));
        self.refresh_plan_list(cx);
    }

    /// Plan — 「↑ 上移」 / 「↓ 下移」chip on item `idx`. `dir` is -1 or 1.
    fn item_move(&mut self, cx: &mut Cx, idx: u64, dir: i32) {
        let n = state().current_plan_items.len() as i64;
        let new_idx = idx as i64 + dir as i64;
        if new_idx < 0 || new_idx >= n {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, &format!("⚠ 已到{}端,无法移动",
                    if dir < 0 { "顶" } else { "底" }));
            return;
        }
        {
            let mut s = state();
            let items = &mut s.current_plan_items;
            items.swap(idx as usize, new_idx as usize);
        }
        let dir_label = if dir < 0 { "上移" } else { "下移" };
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 第 {} 条{} 1 位", idx + 1, dir_label));
        self.refresh_plan_list(cx);
    }

    /// Plan — 「✎ 编辑」chip on item `idx`. Sets `editing_k = idx` so the
    /// next pass of `refresh_plan_list` can show the editor card. (C12
    /// simplification: no live TextInput; shows the values as text and
    /// logs the action; real TextInput card is C12.1.)
    /// Plan — 「✎ 编辑」chip on item `idx`. C16: now really opens the
    /// 1-6 field TextInput editor card. Populates the 6 TextInputs from
    /// the current item, labels them per `item_fields(plan_kind)`, and
    /// shows `plan_editor_card` (hides it when `editing_k == -1`).
    fn item_start_edit(&mut self, cx: &mut Cx, idx: u64) {
        use octostudio_core::item_fields;
        state().editing_k = idx as i64;
        let n = state().current_plan_items.len();
        if (idx as usize) >= n {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 越界");
            state().editing_k = -1;
            self.ui.view(cx, ids!(plan_editor_card)).set_visible(cx, false);
            return;
        }
        let fields = item_fields(state().plan_kind);
        // Populate the 6 TextInputs (fieldN gets the value of the Nth
        // field, empty string for missing fields beyond the schema).
        let item_keys: Vec<String> = (0..6).map(|i| {
            fields.get(i).map(|f| f.key.clone()).unwrap_or_default()
        }).collect();
        let field_ids = [
            ids!(plan_field0), ids!(plan_field1), ids!(plan_field2),
            ids!(plan_field3), ids!(plan_field4), ids!(plan_field5),
        ];
        let labels: Vec<String> = (0..6).map(|i| {
            fields.get(i).map(|f| f.label.clone()).unwrap_or_default()
        }).collect();
        // Build the editor title showing field labels
        let title_labels: Vec<String> = (0..6).filter_map(|i| {
            if labels[i].is_empty() { None }
            else { Some(format!("[{}]{}", i, labels[i])) }
        }).collect();
        let mut title = format!("✎ 编辑第 {} 条 · 字段:", idx + 1);
        if !title_labels.is_empty() {
            title.push_str(&title_labels.join(" "));
        }
        self.ui.label(cx, ids!(plan_editor_title)).set_text(cx, &title);

        if let Some(it) = state().current_plan_items.get(idx as usize) {
            for i in 0..6 {
                let v: String = if item_keys[i].is_empty() {
                    String::new()
                } else {
                    it.get(&item_keys[i]).to_string()
                };
                self.ui.text_input(cx, field_ids[i])
                    .set_text(cx, &v);
            }
        }
        // Show the editor card
        self.ui.view(cx, ids!(plan_editor_card)).set_visible(cx, true);
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✎ 编辑第 {} 条(写入 / 取消 见下方 card)", idx + 1));
        self.ui.redraw(cx);
    }

    /// Plan — 「💾 写入」 in editor card. Reads 6 TextInput fields,
    /// writes back to `state().current_plan_items[editing_k].fields`,
    /// then closes the editor (editing_k = -1).
    fn plan_editor_save(&mut self, cx: &mut Cx) {
        use octostudio_core::item_fields;
        let idx = state().editing_k;
        if idx < 0 {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 没在编辑状态");
            return;
        }
        let fields = item_fields(state().plan_kind);
        let field_ids = [
            ids!(plan_field0), ids!(plan_field1), ids!(plan_field2),
            ids!(plan_field3), ids!(plan_field4), ids!(plan_field5),
        ];
        let item_keys: Vec<String> = (0..6).map(|i| {
            fields.get(i).map(|f| f.key.clone()).unwrap_or_default()
        }).collect();
        let new_values: Vec<String> = (0..6).map(|i| {
            if item_keys[i].is_empty() { return String::new(); }
            self.ui.text_input(cx, field_ids[i]).text()
        }).collect();
        let n_written = {
            let mut s = state();
            if let Some(item) = s.current_plan_items.get_mut(idx as usize) {
                for (i, key) in item_keys.iter().enumerate() {
                    if key.is_empty() { continue; }
                    item.fields.insert(key.clone(), new_values[i].clone());
                }
                item.fields.len()
            } else {
                0
            }
        };
        state().editing_k = -1;
        self.ui.view(cx, ids!(plan_editor_card)).set_visible(cx, false);
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 第 {} 条已写入({} 字段)", idx + 1, n_written));
        self.refresh_plan_list(cx);
    }

    /// Plan — 「✗ 取消」 in editor card. Closes the editor.
    fn plan_editor_cancel(&mut self, cx: &mut Cx) {
        state().editing_k = -1;
        self.ui.view(cx, ids!(plan_editor_card)).set_visible(cx, false);
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, "✗ 取消编辑");
        self.ui.redraw(cx);
    }

    /// Plan — 「💾 保存到 works.json」. Builds a `Work` from
    /// `current_*` state and upserts into `works.json`.
    fn save_plan_to_works(&mut self, cx: &mut Cx) {
        use octostudio_core::{PlanKind, Work};
        if state().current_plan_items.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 无条目可保存");
            return;
        }
        // Allocate next id and stamp time
        let next_id;
        let updated;
        {
            let mut s = state();
            s.seq = s.seq.wrapping_add(1);
            next_id = if s.current_id.is_empty() { format!("w-{}", s.seq) } else { s.current_id.clone() };
            updated = s.seq;
        }
        let kind = state().plan_kind;
        let work = Work {
            id: next_id.clone(),
            title: if state().current_title.is_empty() { "未命名作品".to_string() } else { state().current_title.clone() },
            source: state().current_source.clone(),
            reference: state().current_reference.clone(),
            content: state().current_content.clone(),
            style: state().current_style.clone(),
            prompt: state().current_prompt.clone(),
            plan_kind: Some(kind),
            plan_title: state().current_plan_title.clone(),
            plan_summary: state().current_plan_summary.clone(),
            plan_items: state().current_plan_items.clone(),
            plan_format: octostudio_core::ExportFormat::Markdown,
            theme: state().current_theme.clone(),
            composition: state().current_composition.clone(),
            tags: vec![],
            history: vec![],
            updated,
        };
        {
            let mut s = state();
            if let Some(existing) = s.works.iter_mut().find(|w| w.id == next_id) {
                *existing = work.clone();
            } else {
                s.works.push(work.clone());
            }
            s.current_id = next_id.clone();
        }
        let works = state().works.clone();
        match octostudio_storage::works::save_works(&works) {
            Ok(_) => {
                let plan_kind_label = match kind {
                    PlanKind::None => "原文二创",
                    k => k.short(),
                };
                self.ui.label(cx, ids!(plan_summary_label))
                    .set_text(cx, &format!("{} ({} 条,{})", next_id, work.plan_items.len(), plan_kind_label));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ 已保存 {} 到 works.json,共 {} 件", next_id, works.len()));
            }
            Err(e) => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ 保存失败: {e}"));
            }
        }
        self.refresh_plan_list(cx);
    }

    /// Plan — refresh the 8 body labels from `current_plan_items`.
    /// Slots beyond `len()` show `(空)`.
    fn refresh_plan_list(&mut self, cx: &mut Cx) {
        use octostudio_core::item_fields;
        let items = state().current_plan_items.clone();
        let fields = item_fields(state().plan_kind);
        // Compose one-line summary per item
        let summaries: Vec<String> = items.iter().map(|it| {
            let mut line = String::new();
            for f in &fields {
                let v = it.get(&f.key);
                if v.is_empty() { continue; }
                let s = if v.chars().count() > 18 {
                    let mut s: String = v.chars().take(18).collect();
                    s.push('…');
                    s
                } else {
                    v.to_string()
                };
                line.push_str(&format!("{}:{} | ", f.label, s));
            }
            if line.is_empty() { "(空)".to_string() } else { line }
        }).collect();
        let body_ids = [
            ids!(plan_item0_body), ids!(plan_item1_body), ids!(plan_item2_body),
            ids!(plan_item3_body), ids!(plan_item4_body), ids!(plan_item5_body),
            ids!(plan_item6_body), ids!(plan_item7_body),
        ];
        for (i, id) in body_ids.iter().enumerate() {
            let text = summaries.get(i).cloned().unwrap_or_else(|| "(空)".to_string());
            self.ui.label(cx, *id).set_text(cx, &text);
        }
        let summary = if items.is_empty() {
            "(无计划)".to_string()
        } else {
            format!("{} · 共 {} 条", state().current_plan_title, items.len())
        };
        self.ui.label(cx, ids!(plan_summary_label)).set_text(cx, &summary);
        self.ui.redraw(cx);
    }

    // -------- C13: Studio AI panel + 8 style + 创作/重试/撤销/快照 --------

    /// Sync the title text input from `current_title` (used in handle_startup).
    fn sync_studio_inputs(&mut self, cx: &mut Cx) {
        let title = state().current_title.clone();
        if !title.is_empty() {
            self.ui.text_input(cx, ids!(studio_title_input))
                .set_text(cx, &title);
        }
        let src = if state().current_source.is_empty() {
            state().current_content.clone()
        } else {
            state().current_source.clone()
        };
        let src_label = if src.is_empty() {
            "(空 — 在「录入」填)".to_string()
        } else {
            let n = src.chars().count();
            let preview: String = src.chars().take(120).collect();
            format!("{} 字 · {}{}", n, preview, if n > 120 { "…" } else { "" })
        };
        self.ui.label(cx, ids!(studio_source_label))
            .set_text(cx, &src_label);
        self.ui.redraw(cx);
    }

    /// Sync the title text input → state. Called after any chip click.
    fn write_studio_title_to_state(&mut self, cx: &mut Cx) {
        let title = self.ui.text_input(cx, ids!(studio_title_input)).text();
        let title = title.trim().to_string();
        if !title.is_empty() {
            state().current_title = title;
        }
    }

    /// Switch `current_style` (C13: state-only; chip tinting via script_mod
    /// draw_bg.color requires `apply_over` which ButtonRef doesn't expose,
    /// so the next render pass will show the active style in the status).
    fn studio_set_style(&mut self, cx: &mut Cx, style: &str) {
        state().current_style = style.to_string();
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 风格:{style}"));
        self.ui.redraw(cx);
    }

    /// 1 chip → 1 TextClient panel helper call → 1 result line in
    /// `ai_result_label`. With key: real API; without: friendly hint.
    /// C15: Arm the 70s AI watchdog (splash `arm_watchdog` 1:1).
    /// The actual reset happens when `Event::Timer(te)` arrives in
    /// [`App::handle_event`] — see `handle_watchdog_timer_fire()`.
    fn arm_watchdog(&mut self, cx: &mut Cx) {
        // Disarm any previous timer first.
        self.disarm_watchdog(cx);
        if !state().busy { return; }
        let timer = cx.start_timeout(70.0);
        state().watchdog_timer = Some(timer);
    }

    /// C15: Cancel the watchdog. Call at the end of every AI handler
    /// (success + error) so a fast response doesn't fire the 70s reset.
    fn disarm_watchdog(&mut self, cx: &mut Cx) {
        if let Some(timer) = state().watchdog_timer.take() {
            cx.stop_timer(timer);
        }
    }

    /// C15: When a Timer event arrives, check if it's our watchdog.
    /// If `busy` is still true (the API call hasn't returned in 70s),
    /// reset to false and surface a user-visible "AI 响应超时" status.
    fn handle_watchdog_timer_fire(&mut self, cx: &mut Cx, timer_id: u64) {
        let matches = state()
            .watchdog_timer
            .as_ref()
            .map(|t| t.0 == timer_id)
            .unwrap_or(false);
        if !matches { return; }
        state().busy = false;
        state().watchdog_timer = None;
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, "AI 响应超时(>70 秒),已复位 — 请再点一次");
        self.ui.redraw(cx);
    }

    fn ai_panel_handler(&mut self, cx: &mut Cx, panel: StudioPanel) {
        use octostudio_ai::TextClient;
        self.write_studio_title_to_state(cx);
        // C15: arm the 70s watchdog before any potentially long API call
        state().busy = true;
        self.arm_watchdog(cx);
        let source = {
            let s = state();
            if s.current_source.is_empty() { s.current_content.clone() } else { s.current_source.clone() }
        };
        if source.trim().is_empty() {
            self.ui.label(cx, ids!(ai_result_label))
                .set_text(cx, "⚠ 先在「录入」填原文/内容,或在 title 输入文字");
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 无源内容");
            return;
        }
        let key = state().api_key.clone();
        let key = match key.filter(|k| !k.is_empty()) {
            Some(k) => k,
            None => {
                let demo = match panel {
                    StudioPanel::GenTitle => "清晨四点五十分,海面还蒙着夜的薄纱。",
                    StudioPanel::ExtractKeywords => "海浪;薄纱;日出;节奏;疗愈",
                    StudioPanel::Summarize => "一作者用慢生活的三个时刻,说明'慢'不是懒而是感官。",
                    StudioPanel::StyleVariants => "[理性] 慢 = 时间的有效配置。\n[抒情] 慢 = 把时间还给了感觉。\n[犀利] 慢 = 不被算法消费的主动权。\n[温暖] 慢 = 一杯凉茶里看见的日子。",
                    StudioPanel::TranslateZhEn => "{ zh: \"...\", en: \"...\" }",
                    StudioPanel::ScoreTitle => "85/100 — 建议:加入数字/反差提升点击率",
                    StudioPanel::Budget => "见 设置 屏配额卡片(usage.json)",
                };
                self.ui.label(cx, ids!(ai_result_label))
                    .set_text(cx, &format!("[demo · 无 API key] {} — {}", panel.label(), demo));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ {} (demo)", panel.label()));
                return;
            }
        };
        let client = TextClient::new(key);
        self.ui.label(cx, ids!(ai_result_label))
            .set_text(cx, &format!("🤖 {} 中…(10-30s)", panel.label()));
        self.ui.redraw(cx);
        let result = match panel {
            StudioPanel::GenTitle => client.ai_gen_title(&source).map(format_value),
            StudioPanel::ExtractKeywords => client.ai_extract_keywords(&source).map(format_value),
            StudioPanel::Summarize => client.ai_summarize(&source).map(format_value),
            StudioPanel::StyleVariants => client.ai_style_variants(&source).map(format_value),
            StudioPanel::TranslateZhEn => client.ai_translate_zh_en(&source).map(format_value),
            StudioPanel::ScoreTitle => client.ai_score_title(&source).map(format_value),
            StudioPanel::Budget => Ok("(见 Settings 屏 — 本地统计)".to_string()),
        };
        match result {
            Ok(rendered) => {
                let preview = if rendered.chars().count() > 480 {
                    let mut s: String = rendered.chars().take(480).collect();
                    s.push_str("…(截断)");
                    s
                } else { rendered };
                let final_text = format!("✓ {} — {}", panel.label(), preview);
                self.ui.label(cx, ids!(ai_result_label)).set_text(cx, &final_text);
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ {} 完成", panel.label()));
            }
            Err(e) => {
                self.ui.label(cx, ids!(ai_result_label))
                    .set_text(cx, &format!("✗ {} 失败: {}", panel.label(), e));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ {} 失败: {}", panel.label(), e));
            }
        }
        // C15: disarm the 70s watchdog
        state().busy = false;
        self.disarm_watchdog(cx);
        self.ui.redraw(cx);
    }

    /// AI 创作 — 按 current_style 把 source 改写 → current_content
    fn studio_ai_write_handler(&mut self, cx: &mut Cx) {
        use octostudio_ai::TextClient;
        self.write_studio_title_to_state(cx);
        let source = {
            let s = state();
            if s.current_source.is_empty() { s.current_content.clone() } else { s.current_source.clone() }
        };
        if source.trim().is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 无源内容");
            return;
        }
        let style = state().current_style.clone();
        let sys = format!("你是一名中文写作助手,按「{style}」风格改写用户内容。");
        let key = state().api_key.clone();
        let key = match key.filter(|k| !k.is_empty()) {
            Some(k) => k,
            None => {
                let demo = format!("[demo · {style}]\n\n{source}");
                state().current_content = demo.clone();
                self.ui.label(cx, ids!(ai_result_label))
                    .set_text(cx, &demo);
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ AI 创作完成(demo,{style})"));
                return;
            }
        };
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("🤖 AI 创作中…(style={style})"));
        self.ui.redraw(cx);
        let client = TextClient::new(key);
        match client.chat(&sys, &source) {
            Ok(reply) => {
                state().current_content = reply.clone();
                let preview: String = reply.chars().take(400).collect();
                self.ui.label(cx, ids!(ai_result_label))
                    .set_text(cx, &format!("✓ AI 创作({style}):\n{preview}"));
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ AI 创作完成({})", &style));
            }
            Err(e) => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✗ AI 创作失败: {e}"));
            }
        }
        // C15: disarm
        state().busy = false;
        self.disarm_watchdog(cx);
        self.ui.redraw(cx);
    }

    /// 重试 — 重新跑上一次的 AI 操作(简化:重跑当前 style 改写)
    fn studio_ai_retry_handler(&mut self, cx: &mut Cx) {
        self.studio_ai_write_handler(cx);
    }

    /// 撤销 — 从 undo_stack 弹一个 snapshot 写回 current_content
    fn studio_undo_handler(&mut self, cx: &mut Cx) {
        let popped = {
            let mut s = state();
            s.undo_stack.pop_back()
        };
        match popped {
            Some(snap) => {
                let label = match snap.kind {
                    octostudio_render::state::UndoKind::Title => {
                        state().current_title = snap.value;
                        "标题"
                    }
                    octostudio_render::state::UndoKind::Source => {
                        state().current_source = snap.value;
                        "原文"
                    }
                    octostudio_render::state::UndoKind::Content => {
                        state().current_content = snap.value;
                        "成稿"
                    }
                };
                let n = state().undo_stack.len();
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, &format!("✓ 撤销:{label} 1 步,剩 {n} 步"));
                self.sync_studio_inputs(cx);
            }
            None => {
                self.ui.label(cx, ids!(status_label))
                    .set_text(cx, "⚠ 撤销栈空");
            }
        }
    }

    /// 快照原文 — push current_source → undo_stack,标记 "快照 N"
    fn studio_snapshot_handler(&mut self, cx: &mut Cx) {
        let source = state().current_source.clone();
        if source.is_empty() {
            self.ui.label(cx, ids!(status_label))
                .set_text(cx, "⚠ 原文为空,无可快照内容");
            return;
        }
        let next = {
            let mut s = state();
            s.undo_stack.push_back(octostudio_render::state::UndoSnapshot {
                kind: octostudio_render::state::UndoKind::Source,
                value: source,
            });
            s.undo_stack.len()
        };
        self.ui.label(cx, ids!(status_label))
            .set_text(cx, &format!("✓ 快照原文,撤销栈 {next} 条"));
    }
}

    /// Render a serde_json::Value as a compact one-line string for the
/// panel result label.
fn format_value(v: serde_json::Value) -> String {
    serde_json::to_string_pretty(&v).unwrap_or_else(|_| format!("{v:?}"))
}

// -------- C14: Export 8 格式 chip + 大 Card 预览 + 复制 --------

/// Snapshot the current plan into a `Work` view for export dispatch.
/// Reuses the demo works[0] if state().current_plan_items is empty
/// (so the Export screen always has something to render).
fn current_export_work() -> octostudio_core::Work {
    use octostudio_core::{ExportFormat, Work};
    let s = state();
    let items = if s.current_plan_items.is_empty() {
        s.works.first().map(|w| w.plan_items.clone()).unwrap_or_default()
    } else {
        s.current_plan_items.clone()
    };
    let title = if s.current_plan_title.is_empty() {
        s.works.first().map(|w| w.plan_title.clone()).unwrap_or_else(|| "未命名".to_string())
    } else { s.current_plan_title.clone() };
    let summary = if s.current_plan_summary.is_empty() {
        s.works.first().map(|w| w.plan_summary.clone()).unwrap_or_default()
    } else { s.current_plan_summary.clone() };
    let id = if s.current_id.is_empty() { "w-current".into() } else { s.current_id.clone() };
    let kind = s.plan_kind;
    Work {
        id,
        title,
        source: s.current_source.clone(),
        reference: s.current_reference.clone(),
        content: s.current_content.clone(),
        style: s.current_style.clone(),
        prompt: s.current_prompt.clone(),
        plan_kind: Some(kind),
        plan_title: s.current_plan_title.clone(),
        plan_summary: summary,
        plan_items: items,
        plan_format: ExportFormat::Markdown,
        theme: s.current_theme.clone(),
        composition: s.current_composition.clone(),
        tags: vec![],
        history: vec![],
        updated: 0,
    }
}

impl App {
    fn export_handler(&mut self, cx: &mut Cx, fmt_id: &str) {
        use octostudio_core::ExportFormat;
        use octostudio_export::render_plan_as_text;
        let fmt = match fmt_id {
            "markdown" => ExportFormat::Markdown,
            "wechat" => ExportFormat::Wechat,
            "notion" => ExportFormat::Notion,
            "srt" => ExportFormat::Srt,
            "pack" => ExportFormat::Pack,
            "marp" => ExportFormat::Marp,
            "markmap" => ExportFormat::Markmap,
            _ => ExportFormat::Markdown,
        };
        let work = current_export_work();
        let rendered = render_plan_as_text(&work);
        let label = format!(
            "格式: {} · {} 字",
            match fmt {
                ExportFormat::Markdown => "Markdown",
                ExportFormat::Wechat => "公众号",
                ExportFormat::Notion => "Notion",
                ExportFormat::Srt => "SRT 字幕",
                ExportFormat::Pack => "视频制作包",
                ExportFormat::Marp => "Marp 演示",
                ExportFormat::Markmap => "Markmap 导图",
            },
            rendered.chars().count()
        );
        let preview = if rendered.chars().count() > 1800 {
            let mut s: String = rendered.chars().take(1800).collect();
            s.push_str("\n…(截断 1800 字)");
            s
        } else { rendered };
        self.ui.label(cx, ids!(export_preview_label)).set_text(cx, &preview);
        self.ui.label(cx, ids!(export_status_label)).set_text(cx, &label);
        self.ui.label(cx, ids!(export_hint_label)).set_text(
            cx,
            &format!("提示: 复制按钮可写入剪贴板 · 当前 plan_kind = {}",
                     work.plan_kind.unwrap_or(octostudio_core::PlanKind::None).short())
        );
        self.ui.redraw(cx);
    }

    /// 全部预览 — 一次渲染所有 7 个格式, 大 Card 拼起来
    fn export_all_handler(&mut self, cx: &mut Cx) {
        use octostudio_core::ExportFormat;
        use octostudio_export::render_plan_as_text;
        let work = current_export_work();
        let mut body = String::new();
        for fmt in [
            ExportFormat::Markdown, ExportFormat::Wechat, ExportFormat::Notion,
            ExportFormat::Srt, ExportFormat::Pack, ExportFormat::Marp,
            ExportFormat::Markmap,
        ] {
            let mut w = work.clone();
            w.plan_format = fmt;
            let rendered = render_plan_as_text(&w);
            let n = rendered.chars().count();
            body.push_str(&format!("\n── {} ({} 字) ──\n{}\n",
                match fmt {
                    ExportFormat::Markdown => "Markdown",
                    ExportFormat::Wechat => "公众号",
                    ExportFormat::Notion => "Notion",
                    ExportFormat::Srt => "SRT 字幕",
                    ExportFormat::Pack => "视频制作包",
                    ExportFormat::Marp => "Marp 演示",
                    ExportFormat::Markmap => "Markmap 导图",
                },
                n,
                if n > 600 {
                    let mut s: String = rendered.chars().take(600).collect();
                    s.push_str("…(截断)");
                    s
                } else { rendered }
            ));
        }
        // Total body length cap
        let total_n = body.chars().count();
        if total_n > 3600 {
            let mut s: String = body.chars().take(3600).collect();
            s.push_str("\n…(总 7 格式,截断 3600 字)");
            body = s;
        }
        self.ui.label(cx, ids!(export_preview_label)).set_text(cx, &body);
        self.ui.label(cx, ids!(export_status_label))
            .set_text(cx, &format!("全部 7 格式 · 总 {} 字", total_n.min(3600)));
        self.ui.redraw(cx);
    }

    /// 复制 — 优先调 cx.copy_to_clipboard(makepad 的剪贴板 API);
    /// 失败 fallback 到写 tmpdir 文件,status 行明示用户去哪取。
    fn export_copy_handler(&mut self, cx: &mut Cx) {
        use octostudio_export::render_plan_as_text;
        let work = current_export_work();
        let rendered = render_plan_as_text(&work);
        let n = rendered.chars().count();
        cx.copy_to_clipboard(&rendered);
        // Always also write a tmpdir fallback so the export is never lost.
        let fallback = std::env::temp_dir().join("octostudio-export.txt");
        match std::fs::write(&fallback, &rendered) {
            Ok(_) => {
                self.ui.label(cx, ids!(export_status_label)).set_text(
                    cx,
                    &format!("✓ 复制 {n} 字 → 剪贴板 + 备份 {}", fallback.display())
                );
            }
            Err(e) => {
                self.ui.label(cx, ids!(export_status_label))
                    .set_text(cx, &format!("⚠ 剪贴板调用过,备份失败: {e}"));
            }
        }
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
        self.sync_studio_inputs(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 侧边导航 — off/on 两态都接同一 goto
        if self.ui.button(cx, ids!(nav_home_off)).clicked(actions)
            || self.ui.button(cx, ids!(nav_home_on)).clicked(actions)
        {
            self.goto(cx, Screen::Home);
            return;
        }
        if self.ui.button(cx, ids!(nav_compose_off)).clicked(actions)
            || self.ui.button(cx, ids!(nav_compose_on)).clicked(actions)
        {
            self.goto(cx, Screen::Compose);
            return;
        }
        if self.ui.button(cx, ids!(nav_studio_off)).clicked(actions)
            || self.ui.button(cx, ids!(nav_studio_on)).clicked(actions)
        {
            self.goto(cx, Screen::Studio);
            return;
        }
        if self.ui.button(cx, ids!(nav_plan_off)).clicked(actions)
            || self.ui.button(cx, ids!(nav_plan_on)).clicked(actions)
        {
            self.goto(cx, Screen::Plan);
            return;
        }
        if self.ui.button(cx, ids!(nav_settings_off)).clicked(actions)
            || self.ui.button(cx, ids!(nav_settings_on)).clicked(actions)
        {
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
        if self.ui.button(cx, ids!(plan_item_add)).clicked(actions) {
            self.item_add(cx);
            return;
        }
        if self.ui.button(cx, ids!(plan_save_works)).clicked(actions) {
            self.save_plan_to_works(cx);
            return;
        }
        if self.ui.button(cx, ids!(plan_to_export)).clicked(actions) {
            self.goto(cx, Screen::Export);
            return;
        }
        if self.ui.button(cx, ids!(plan_editor_save)).clicked(actions) {
            self.plan_editor_save(cx);
            return;
        }
        if self.ui.button(cx, ids!(plan_editor_cancel)).clicked(actions) {
            self.plan_editor_cancel(cx);
            return;
        }
        // Studio — 7 AI panel chips
        if self.ui.button(cx, ids!(studio_p1)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::GenTitle);
            return;
        }
        if self.ui.button(cx, ids!(studio_p2)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::ExtractKeywords);
            return;
        }
        if self.ui.button(cx, ids!(studio_p3)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::Summarize);
            return;
        }
        if self.ui.button(cx, ids!(studio_p4)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::StyleVariants);
            return;
        }
        if self.ui.button(cx, ids!(studio_p5)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::TranslateZhEn);
            return;
        }
        if self.ui.button(cx, ids!(studio_p6)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::ScoreTitle);
            return;
        }
        if self.ui.button(cx, ids!(studio_p7)).clicked(actions) {
            self.ai_panel_handler(cx, StudioPanel::Budget);
            return;
        }
        if self.ui.button(cx, ids!(studio_ai_write)).clicked(actions) {
            self.studio_ai_write_handler(cx);
            return;
        }
        if self.ui.button(cx, ids!(studio_ai_retry)).clicked(actions) {
            self.studio_ai_retry_handler(cx);
            return;
        }
        if self.ui.button(cx, ids!(studio_undo)).clicked(actions) {
            self.studio_undo_handler(cx);
            return;
        }
        if self.ui.button(cx, ids!(studio_snapshot)).clicked(actions) {
            self.studio_snapshot_handler(cx);
            return;
        }
        // 8 style chips
        if self.ui.button(cx, ids!(studio_s1)).clicked(actions) { self.studio_set_style(cx, "改写"); return; }
        if self.ui.button(cx, ids!(studio_s2)).clicked(actions) { self.studio_set_style(cx, "翻译"); return; }
        if self.ui.button(cx, ids!(studio_s3)).clicked(actions) { self.studio_set_style(cx, "总结"); return; }
        if self.ui.button(cx, ids!(studio_s4)).clicked(actions) { self.studio_set_style(cx, "评论"); return; }
        if self.ui.button(cx, ids!(studio_s5)).clicked(actions) { self.studio_set_style(cx, "润色"); return; }
        if self.ui.button(cx, ids!(studio_s6)).clicked(actions) { self.studio_set_style(cx, "续写"); return; }
        if self.ui.button(cx, ids!(studio_s7)).clicked(actions) { self.studio_set_style(cx, "扩写"); return; }
        if self.ui.button(cx, ids!(studio_s8)).clicked(actions) { self.studio_set_style(cx, "小红书体"); return; }
        // Export — 8 格式 chip + 复制 + 返回
        if self.ui.button(cx, ids!(export_f1)).clicked(actions) { self.export_handler(cx, "markdown"); return; }
        if self.ui.button(cx, ids!(export_f2)).clicked(actions) { self.export_handler(cx, "wechat"); return; }
        if self.ui.button(cx, ids!(export_f3)).clicked(actions) { self.export_handler(cx, "notion"); return; }
        if self.ui.button(cx, ids!(export_f4)).clicked(actions) { self.export_handler(cx, "srt"); return; }
        if self.ui.button(cx, ids!(export_f5)).clicked(actions) { self.export_handler(cx, "pack"); return; }
        if self.ui.button(cx, ids!(export_f6)).clicked(actions) { self.export_handler(cx, "marp"); return; }
        if self.ui.button(cx, ids!(export_f7)).clicked(actions) { self.export_handler(cx, "markmap"); return; }
        if self.ui.button(cx, ids!(export_f8)).clicked(actions) { self.export_all_handler(cx); return; }
        if self.ui.button(cx, ids!(export_copy)).clicked(actions) { self.export_copy_handler(cx); return; }
        if self.ui.button(cx, ids!(export_back)).clicked(actions) { self.goto(cx, Screen::Plan); return; }
        // Per-item 4 chips × 8 slots — 32 explicit button checks.
        for (i, ids_arr, acts) in [
            (0u64, [ids!(plan_item0_edit), ids!(plan_item0_up), ids!(plan_item0_down), ids!(plan_item0_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (1u64, [ids!(plan_item1_edit), ids!(plan_item1_up), ids!(plan_item1_down), ids!(plan_item1_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (2u64, [ids!(plan_item2_edit), ids!(plan_item2_up), ids!(plan_item2_down), ids!(plan_item2_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (3u64, [ids!(plan_item3_edit), ids!(plan_item3_up), ids!(plan_item3_down), ids!(plan_item3_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (4u64, [ids!(plan_item4_edit), ids!(plan_item4_up), ids!(plan_item4_down), ids!(plan_item4_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (5u64, [ids!(plan_item5_edit), ids!(plan_item5_up), ids!(plan_item5_down), ids!(plan_item5_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (6u64, [ids!(plan_item6_edit), ids!(plan_item6_up), ids!(plan_item6_down), ids!(plan_item6_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
            (7u64, [ids!(plan_item7_edit), ids!(plan_item7_up), ids!(plan_item7_down), ids!(plan_item7_del)],
                 [PlanItemAction::Edit, PlanItemAction::Up, PlanItemAction::Down, PlanItemAction::Del]),
        ] {
            for (j, id) in ids_arr.iter().enumerate() {
                if self.ui.button(cx, *id).clicked(actions) {
                    self.plan_item_action(cx, i, acts[j]);
                    return;
                }
            }
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
        // C15: 70s watchdog timer fire handler
        if let Event::Timer(te) = event {
            self.handle_watchdog_timer_fire(cx, te.timer_id);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    /// Initialize the 5 tab buttons with their proper emoji + label.
    /// Called from handle_startup after the widget tree is built.
    fn init_tabs(&mut self, cx: &mut Cx) {
        let tabs: &[(&[LiveId], &str)] = &[
            (ids!(nav_home_off),     "◐  广场"),
            (ids!(nav_home_on),      "◐  广场"),
            (ids!(nav_compose_off),  "✎  录入"),
            (ids!(nav_compose_on),   "✎  录入"),
            (ids!(nav_studio_off),   "◇  工坊"),
            (ids!(nav_studio_on),    "◇  工坊"),
            (ids!(nav_plan_off),     "▤  计划"),
            (ids!(nav_plan_on),      "▤  计划"),
            (ids!(nav_settings_off), "⚙  设置"),
            (ids!(nav_settings_on),  "⚙  设置"),
        ];
        for (id, label) in tabs.iter() {
            self.ui.button(cx, id).set_text(cx, label);
        }
        self.apply_nav(cx, Screen::Home);
        self.refresh_nav_status(cx);
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
                self.refresh_nav_status(cx);
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
                self.refresh_nav_status(cx);
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
        // C15: disarm
        state().busy = false;
        self.disarm_watchdog(cx);
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
        // C15: disarm
        state().busy = false;
        self.disarm_watchdog(cx);
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

// -------- C12: Plan view action enum + per-slot handler --------

#[derive(Debug, Clone, Copy)]
enum PlanItemAction { Edit, Up, Down, Del }

// -------- C13: Studio 7 panel chip enum --------

#[derive(Debug, Clone, Copy)]
enum StudioPanel {
    GenTitle,
    ExtractKeywords,
    Summarize,
    StyleVariants,
    TranslateZhEn,
    ScoreTitle,
    Budget,
}

impl StudioPanel {
    fn label(self) -> &'static str {
        match self {
            StudioPanel::GenTitle => "起标题",
            StudioPanel::ExtractKeywords => "关键词",
            StudioPanel::Summarize => "摘要",
            StudioPanel::StyleVariants => "风格迁移",
            StudioPanel::TranslateZhEn => "中英对照",
            StudioPanel::ScoreTitle => "标题打分",
            StudioPanel::Budget => "模型预算",
        }
    }
}
