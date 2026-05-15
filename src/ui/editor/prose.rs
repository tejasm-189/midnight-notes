use crate::core::markdown;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn ProseEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();
    let mut show_preview = use_signal(|| true);

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0; overflow: hidden;",
            button {
                style: "align-self: flex-end; background: {c.bg_surface}; border: 1px solid {c.border};
                        color: {c.text_secondary}; padding: 4px 8px; border-radius: 4px; cursor: pointer;
                        font-family: 'JetBrains Mono', monospace; font-size: 11px; margin-bottom: 8px;",
                onclick: move |_| show_preview.set(!show_preview()),
                if *show_preview.read() { "Hide Preview" } else { "Show Preview" }
            }
            div { style: "flex: 1; display: flex; gap: 16px; min-height: 0; overflow: hidden;",
                if !*show_preview.read() || content.is_empty() {
                    textarea {
                        value: "{content}",
                        oninput: move |e| oninput.call(e.value()),
                        placeholder: "Start writing...",
                        style: "flex: 1; background: transparent; border: none; color: {c.text_primary};
                                font-family: Inter; font-size: 16px; line-height: 1.7; resize: none;
                                outline: none; padding: 0; min-height: 300px;",
                    }
                }
                if *show_preview.read() && !content.is_empty() {
                    div {
                        style: "flex: 1; overflow-y: auto; padding: 0 0 0 16px;
                                font-family: Inter; font-size: 16px; line-height: 1.7; color: {c.text_primary};
                                border-left: 1px solid {c.border};",
                        dangerous_inner_html: "{markdown::render_markdown(&content)}",
                    }
                }
            }
        }
    }
}
