use crate::core::markdown;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn ProseEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();
    let html = markdown::render_markdown(&content);
    let starter = "### Integration Checklist\n- [ ] Validate API payload format\n- [ ] Update service logic\n- [ ] Add regression tests\n\n```ts\ninterface AuthPayload {\n  sub: string;\n  scopes: string[];\n}\n```";
    let preview_html = if content.trim().is_empty() {
        markdown::render_markdown(starter)
    } else {
        html
    };

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                span { style: "font-size: 11px; color: {c.text_muted}; font-family: 'JetBrains Mono', monospace;",
                    "Live Preview"
                }
                span { style: "font-size: 11px; color: {c.accent}; font-family: 'JetBrains Mono', monospace;",
                    "Clean writing canvas"
                }
            }

            div { style: "flex: 1; min-height: 0; position: relative; border: 1px solid {c.border}; border-radius: 10px; background: {c.bg_surface}; overflow: hidden;",
                div { style: "position: absolute; inset: 0; overflow-y: auto; padding: 28px; z-index: 1; pointer-events: none;",
                    div { style: "font-family: Inter, -apple-system, sans-serif; font-size: 17px; line-height: 1.8; color: {c.text_primary};", dangerous_inner_html: "{preview_html}" }
                }

                textarea {
                    value: "{content}",
                    oninput: move |e| oninput.call(e.value()),
                    placeholder: "Start writing...",
                    style: "position: absolute; inset: 0; width: 100%; height: 100%; z-index: 2; background: transparent; border: none;
                            color: transparent; caret-color: {c.accent}; font-family: Inter, -apple-system, sans-serif;
                            font-size: 17px; line-height: 1.8; resize: none; outline: none; padding: 28px;",
                }
            }
        }
    }
}
