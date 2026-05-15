use crate::core::markdown;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn NoteViewer(title: String, content: String, on_close: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let html = markdown::render_markdown(&content);

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; background: {c.bg_primary};",
            header { style: "height: 56px; background: {c.bg_canvas}; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",
                h2 { style: "font-family: Inter; font-size: 18px; font-weight: 600; color: {c.text_primary};", "Preview" }
                button { style: "background: none; border: 1px solid {c.border}; color: {c.text_secondary}; padding: 4px 12px; border-radius: 4px; cursor: pointer; font-family: 'JetBrains Mono', monospace; font-size: 11px;",
                    onclick: move |_| on_close.call(()), "Close"
                }
            }
            div { style: "flex: 1; overflow-y: auto; display: flex; justify-content: center; padding: 32px;",
                div { style: "width: 100%; max-width: 720px;",
                    h1 { style: "font-family: Inter; font-size: 28px; font-weight: 700; color: {c.text_primary}; margin-bottom: 24px;", "{title}" }
                    div { style: "font-family: Inter; font-size: 16px; line-height: 1.7; color: {c.text_primary};", dangerous_inner_html: "{html}" }
                }
            }
        }
    }
}
