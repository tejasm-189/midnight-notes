use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn CodeEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();
    let line_count = content.lines().count().max(20);

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 10px;",
                span { style: "font-size: 11px; color: {c.text_muted}; font-family: 'JetBrains Mono', monospace;", "Code Integration" }
                span { style: "font-size: 11px; color: {c.accent}; font-family: 'JetBrains Mono', monospace;", "Checklist + editor" }
            }
            div { style: "flex: 1; min-height: 0; background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 10px; overflow: hidden; display: flex;",
                div { style: "flex: 1; min-width: 0; display: flex; flex-direction: column;",
                    div { style: "height: 34px; display: flex; align-items: center; padding: 0 12px; border-bottom: 1px solid {c.border}; background: {c.bg_surface_low};",
                        span { style: "font-size: 11px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace;", "main.ts" }
                    }
                    div { style: "flex: 1; min-height: 0; display: flex; background: {c.bg_canvas};",
                        div { style: "width: 44px; border-right: 1px solid {c.border}; padding: 12px 8px; overflow: hidden;",
                            {(1..=line_count).map(|i| rsx! {
                                div { key: "{i}", style: "height: 22px; text-align: right; font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_muted};", "{i}" }
                            })}
                        }
                        textarea {
                            value: "{content}",
                            oninput: move |e| oninput.call(e.value()),
                            placeholder: "// Start coding...",
                            spellcheck: false,
                            style: "flex: 1; width: 100%; background: transparent; border: none; color: {c.text_primary};
                                    font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.57; resize: none;
                                    outline: none; padding: 12px 16px; tab-size: 2;",
                        }
                    }
                }
                aside { style: "width: 240px; border-left: 1px solid {c.border}; background: {c.bg_surface_low}; padding: 14px 12px;",
                    h4 { style: "font-family: Inter; font-size: 13px; font-weight: 600; color: {c.text_primary}; margin-bottom: 10px;", "Integration Checklist" }
                    ul { style: "display: flex; flex-direction: column; gap: 8px; margin: 0; padding: 0; list-style: none;",
                        li { style: "display: flex; gap: 8px; align-items: flex-start; font-family: Inter; font-size: 12px; color: {c.text_secondary};", span { style: "color: {c.accent_green};", "✓" } span { "Decode new payload fields" } }
                        li { style: "display: flex; gap: 8px; align-items: flex-start; font-family: Inter; font-size: 12px; color: {c.text_secondary};", span { style: "color: {c.text_muted};", "○" } span { "Map scopes to authorization guards" } }
                        li { style: "display: flex; gap: 8px; align-items: flex-start; font-family: Inter; font-size: 12px; color: {c.text_secondary};", span { style: "color: {c.text_muted};", "○" } span { "Add tests for token compatibility" } }
                    }
                }
            }
        }
    }
}
