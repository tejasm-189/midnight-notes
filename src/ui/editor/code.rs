use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn CodeEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",
            textarea {
                value: "{content}",
                oninput: move |e| oninput.call(e.value()),
                placeholder: "// Start coding...",
                spellcheck: false,
                style: "flex: 1; width: 100%; background: {c.bg_canvas}; border: 1px solid {c.border}; border-radius: 4px;
                        color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.6;
                        resize: none; outline: none; padding: 16px; min-height: 300px;
                        tab-size: 2;",
            }
        }
    }
}
