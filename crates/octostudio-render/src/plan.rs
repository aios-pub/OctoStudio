//! Plan screen — plan items list + composition card + AI 配图 toggle.
//!
//! C3 stub: header + 1 placeholder. C6 wires image AI; C7 wires video AI.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
}
