use crate::core::markdown;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn ProseEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();
    let mut show_preview = use_signal(|| true);

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0; overflow: hidden;",
            div { style: "display: flex; align-items: center; justify-content: flex-end; gap: 8px; margin-bottom: 16px;",
                span { style: "font-family: Inter; font-size: 11px; color: {c.text_muted};",
                    if *show_preview.read() { "Preview Enabled" } else { "Edit Mode" }
                }
                button {
                    style: "background: {c.bg_surface}; border: 1px solid {c.border};
                            color: {c.text_secondary}; width: 28px; height: 28px; border-radius: 50%; cursor: pointer;
                            display: flex; align-items: center; justify-content: center; transition: all 0.2s;",
                    onclick: move |_| show_preview.set(!show_preview()),
                    span { class: "material-symbols-outlined", style: "font-size: 16px;",
                        if *show_preview.read() { "edit" } else { "visibility" }
                    }
                }
            }
            div { style: "flex: 1; display: flex; gap: 16px; min-height: 0; overflow: hidden;",
                textarea {
                    value: "{content}",
                    oninput: move |e| oninput.call(e.value()),
                    placeholder: "Start writing...",
                    style: "flex: 1; background: transparent; border: none; color: {c.text_primary};
                            font-family: Inter; font-size: 16px; line-height: 1.7; resize: none;
                            outline: none; padding: 0; min-height: 300px;",
                }
                if *show_preview.read() && !content.is_empty() {
                    div {
                        style: "flex: 1; overflow-y: auto; padding: 0 0 0 16px;
                                font-family: Inter; font-size: 16px; line-height: 1.7; color: {c.text_primary};
                                border-left: 1px solid {c.border_light};",
                        dangerous_inner_html: "{markdown::render_markdown(&content)}",
                    }
                }
            }
        }
    }
}
