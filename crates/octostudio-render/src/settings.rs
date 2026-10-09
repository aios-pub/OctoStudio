//! Settings screen — API key + test connection + theme + usage stats.
//!
//! C3 stub: header + 1 placeholder. C9 wires API key persistence + connection test.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

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
}
