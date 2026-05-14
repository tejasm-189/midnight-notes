use dioxus::prelude::*;

use crate::core::markdown;

#[component]
pub fn ProseEditor(
    content: String,
    oninput: EventHandler<String>,
    title: String,
    ontitleinput: EventHandler<String>,
) -> Element {
    let mut show_preview = use_signal(|| true);

    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",

            input {
                r#type: "text",
                value: "{title}",
                oninput: move |e| ontitleinput.call(e.value()),
                placeholder: "Untitled",
                style: "width: 100%; background: transparent; border: none;
                        font-family: Inter; font-size: 32px; font-weight: 700;
                        letter-spacing: -0.02em; color: #e5e2e1;
                        padding: 24px 0 16px; outline: none;",
            }

            button {
                style: "align-self: flex-end; background: none; border: 1px solid #3b494b;
                        color: #b9cacb; padding: 4px 8px; border-radius: 4px; cursor: pointer;
                        font-family: 'JetBrains Mono', monospace; font-size: 11px; margin-bottom: 8px;",
                onclick: move |_| show_preview.set(!show_preview()),
                if *show_preview.read() { "Hide Preview" } else { "Show Preview" }
            }

            div {
                style: "flex: 1; display: flex; gap: 16px; min-height: 0; overflow: hidden;",

                if !*show_preview.read() || content.is_empty() {
                    textarea {
                        value: "{content}",
                        oninput: move |e| oninput.call(e.value()),
                        placeholder: "Start writing...",
                        style: "flex: 1; background: transparent; border: none; color: #e5e2e1;
                                font-family: Inter; font-size: 16px; line-height: 1.6; resize: none;
                                outline: none; padding: 0; min-height: 200px;",
                    }
                }

                if *show_preview.read() && !content.is_empty() {
                    div {
                        style: "flex: 1; overflow-y: auto; padding: 0;
                                font-family: Inter; font-size: 16px; line-height: 1.6; color: #e5e2e1;",
                        dangerous_inner_html: "{markdown::render_markdown(&content)}",
                    }
                }
            }
        }
    }
}
