//! Compose screen — scenario / theme / preset chips + three input cards.
//!
//! C3 stub: header + 1 placeholder. C5 wires AI generation pipeline.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
}
