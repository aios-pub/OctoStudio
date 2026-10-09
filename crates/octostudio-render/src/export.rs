//! Export screen — format grid + big preview card.
//!
//! C3 stub: header + 1 placeholder. C8 wires 8 format renderers.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
}
