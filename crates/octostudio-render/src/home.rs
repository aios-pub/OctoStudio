//! Home screen — works library, search, sort, multi-select, scenario grid.
//!
//! C3 stub: header + 1 placeholder Label. C4 wires works.json loading; C5-C7
//! add real sort/filter UI.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
            text: "今天共 N 件作品 · C3 渲染骨架就位"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }

        home_placeholder := Label{
            text: "works 列表 / 搜索 / 排序 chip / 场景网格 — C4-C5 落地"
            draw_text.color: #x8E8E93
            draw_text.text_style.font_size: 12
        }
    }
}
