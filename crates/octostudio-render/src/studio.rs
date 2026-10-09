//! Studio screen — 8 style chips + 7 AI panel chips + content editor.
//!
//! C3 stub: header + 1 placeholder. C5 wires text AI; C9 wires real settings.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
}
