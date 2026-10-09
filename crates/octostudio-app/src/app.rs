//! App shell — widget tree, app entry trait, placeholder state.
//!
//! C1: hello-world window showing the v0.6 banner. Real screens arrive in C3
//! (render crate) and state arrives in C2 (core crate).

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    // Header banner — visible on every screen, replaced by a TabBar in C3.
    let AppHeader = View{
        width: Fill height: Fit
        flow: Down
        spacing: 4
        padding: Inset{top: 16, bottom: 16, left: 24, right: 24}
        draw_bg.color: #xF6F6F8

        title := Label{
            text: "OctoStudio v0.6"
            draw_text.color: #x1c1c1e
            draw_text.text_style.font_size: 26
        }

        subtitle := Label{
            text: "Native Makepad · Agnes 3.0 Flash / Image 2.5 / Video 2.5"
            draw_text.color: #x8e8e93
            draw_text.text_style.font_size: 12
        }
    }

    // Body placeholder — C3 replaces with the 5-screen overlay.
    let AppBody = View{
        width: Fill height: Fill
        flow: Down
        spacing: 16
        padding: Inset{top: 32, bottom: 32, left: 24, right: 24}
        align: Align{x: 0.5, y: 0.5}
        draw_bg.color: #xFFFFFF

        placeholder := Label{
            text: "Scaffold v0.6 — C1 ok. Real screens arrive in C3 (render crate)."
            draw_text.color: #x1c1c1e
            draw_text.text_style.font_size: 14
        }
    }

    // Bottom status line — replaced by 5-tab TabBar in C3.
    let AppFooter = View{
        width: Fill height: Fit
        flow: Right
        spacing: 8
        padding: Inset{top: 8, bottom: 8, left: 24, right: 24}
        align: Align{y: 0.5}
        draw_bg.color: #xF6F6F8

        status_label := Label{
            width: Fill
            text: "就绪 · native scaffold build ok"
            draw_text.color: #x8e8e93
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
                        AppBody{}
                        AppFooter{}
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

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        log!("OctoStudio v0.6 — native Makepad app starting (C1 scaffold)");
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
