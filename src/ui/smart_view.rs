use dioxus::prelude::*;

use crate::core::backlinks::BacklinkService;
use crate::core::search::SearchService;
use crate::ui::theme::use_theme_colors;

#[derive(Clone)]
struct SmartResult {
    note_id: String,
    title: String,
    snippet: String,
    updated_at: String,
}

#[component]
pub fn SmartViewPanel(db: Option<std::sync::Arc<crate::storage::Database>>) -> Element {
    let c = use_theme_colors();
    let mut query = use_signal(String::new);
    let mut results = use_signal(Vec::<SmartResult>::new);
    let mut linked = use_signal(Vec::<String>::new);
    let mut has_searched = use_signal(|| false);

    // Watch query changes and auto-search (debounced via effect)
    let db_eff = db.clone();
    use_effect(move || {
        let q = query.read().clone();
        if q.is_empty() {
            return;
        }
        if let Some(ref d) = db_eff {
            if let Ok(r) = SearchService::new(d).search(&q) {
                let sr: Vec<SmartResult> = r
                    .into_iter()
                    .map(|sr| SmartResult {
                        note_id: sr.note_id,
                        title: sr.title,
                        snippet: sr.snippet,
                        updated_at: sr.updated_at,
                    })
                    .collect();
                results.set(sr.clone());
                has_searched.set(true);
                if let Some(first) = sr.first() {
                    if let Ok(notes) = BacklinkService::new(d).get_linked_mentions(&first.note_id) {
                        linked.set(notes.iter().map(|n| format!("[[{}]]", n.title)).collect());
                    }
                }
            }
        }
    });

    let example_queries = [
        "has:todo",
        "\"error handling\"",
        "path:docs/",
        "tag:architecture NEAR database",
    ];

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: {c.bg_primary};",
            header { style: "height: 56px; background: {c.bg_canvas}; border-bottom: 1px solid {c.border}; display: flex; align-items: center; padding: 0 16px; gap: 12px;",
                span { class: "material-symbols-outlined fill", style: "font-size: 18px; color: {c.accent_green};", "lock" }
                h2 { style: "font-family: Inter; font-size: 18px; font-weight: 600; color: {c.text_primary};", "Smart Views" }
            }
            div { style: "flex: 1; overflow-y: auto; display: flex; justify-content: center; padding-bottom: 40px;",
                div { style: "width: 100%; max-width: 840px; padding: 24px 32px; display: flex; flex-direction: column; gap: 24px;",
                    div { style: "border: 1px solid {c.border}; border-radius: 4px; background: {c.bg_surface}; display: flex; align-items: center; padding: 8px 12px;",
                        span { class: "material-symbols-outlined", style: "font-size: 18px; color: {c.text_secondary}; margin-right: 8px;", "search" }
                        input { r#type: "text", value: "{query}", oninput: move |e| query.set(e.value()),
                            placeholder: "FTS5 Query: e.g. 'API design' NEAR 'authentication'",
                            style: "width: 100%; background: transparent; border: none; color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; font-size: 13px; outline: none;",
                        }
                    }
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap; align-items: center;",
                        span { style: "font-size: 11px; color: {c.text_muted}; font-family: 'JetBrains Mono', monospace; padding: 4px 6px; background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 2px;", "Examples:" }
                        {example_queries.iter().map(|eq| {
                            let q = eq.to_string();
                            rsx! {
                                button { key: "{eq}", style: "font-size: 11px; color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; padding: 4px 8px; background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 2px; cursor: pointer;",
                                    onclick: move |_| query.set(q.clone()),
                                    "{eq}"
                                }
                            }
                        })}
                    }
                    if *has_searched.read() {
                        div { style: "display: grid; grid-template-columns: 2fr 1fr; gap: 16px;",
                            div { style: "display: flex; flex-direction: column; gap: 12px;",
                                h3 { style: "font-size: 11px; color: {c.accent}; font-family: 'JetBrains Mono', monospace; text-transform: uppercase; letter-spacing: 0.08em;", "Matches ({results.read().len()})" }
                                {results.read().iter().map(|r| {
                                    rsx! {
                                        div { key: "{r.note_id}", style: "border: 1px solid {c.border}; background: {c.bg_surface}; border-radius: 4px; padding: 12px; cursor: pointer;",
                                            div { style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                                                h4 { style: "font-family: Inter; font-size: 16px; font-weight: 600; color: {c.text_primary};", "{r.title}" }
                                                span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_secondary};", "{r.updated_at}" }
                                            }
                                            p { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; line-height: 1.4;", "{r.snippet}" }
                                        }
                                    }
                                })}
                                if results.read().is_empty() { div { style: "padding: 24px; text-align: center; font-family: Inter; font-size: 13px; color: {c.text_muted};", "No matches found." } }
                            }
                            div { style: "border: 1px solid {c.border}; background: {c.bg_canvas}; border-radius: 4px; padding: 12px; display: flex; flex-direction: column; gap: 12px;",
                                div { style: "display: flex; align-items: center; gap: 8px; border-bottom: 1px solid {c.border}; padding-bottom: 8px;",
                                    span { class: "material-symbols-outlined", style: "font-size: 18px; color: {c.accent};", "hub" }
                                    h3 { style: "font-size: 11px; color: {c.accent}; font-family: 'JetBrains Mono', monospace; text-transform: uppercase;", "Linked Graph" }
                                }
                                if !linked.read().is_empty() {
                                    p { style: "font-size: 13px; color: {c.text_secondary}; font-family: Inter;", "Nodes connected to selected result" }
                                    {linked.read().iter().map(|l| rsx! {
                                        div { key: "{l}", style: "display: flex; align-items: center; gap: 6px; padding: 4px 0; font-family: 'JetBrains Mono', monospace; font-size: 13px; color: {c.text_primary};",
                                            span { class: "material-symbols-outlined", style: "font-size: 14px; color: {c.text_secondary};", "link" }
                                            span { "{l}" }
                                        }
                                    })}
                                } else { p { style: "font-size: 13px; color: {c.text_muted}; font-family: Inter;", "No linked nodes. Create [[wiki-links]] to see connections." } }
                            }
                        }
                    } else {
                        div { style: "padding: 64px; text-align: center;",
                            span { class: "material-symbols-outlined", style: "font-size: 48px; color: {c.border}; display: block; margin-bottom: 12px;", "auto_awesome" }
                            p { style: "font-family: Inter; font-size: 15px; color: {c.text_muted};", "Click an example query above to search" }
                        }
                    }
                }
            }
            footer { style: "height: 28px; background: {c.bg_canvas}; border-top: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 10px;",
                span { style: "font-weight: 700; color: {c.accent}; display: flex; align-items: center; gap: 4px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 12px; color: {c.accent_green};", "lock" } "End-to-End Encrypted"
                }
                div { style: "display: flex; gap: 16px; color: {c.text_muted};",
                    span { "Word Count: 0" } span { "Read Time: 0m" } span { "VIM Mode: Disabled" }
                }
            }
        }
    }
}
