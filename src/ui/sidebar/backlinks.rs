use crate::core::backlinks::BacklinkService;
use crate::storage::models::Note;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn BacklinksPanel(
    note_id: Option<String>,
    db: Option<std::sync::Arc<crate::storage::Database>>,
    on_open: EventHandler<String>,
) -> Element {
    let c = use_theme_colors();
    let mut mentions = use_signal(Vec::<Note>::new);

    if let Some(ref nid) = note_id {
        if let Some(ref d) = db {
            if let Ok(m) = BacklinkService::new(d).get_linked_mentions(nid) {
                mentions.set(m);
            }
        }
    }

    rsx! {
        div { style: "padding: 8px 16px;",
            h4 { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 6px; font-family: 'JetBrains Mono', monospace;",
                "Linked Mentions ({mentions.read().len()})"
            }
            {mentions.read().iter().map(|m| {
                let mid = m.id.clone();
                rsx! {
                    div { key: "{m.id}", style: "display: flex; align-items: center; gap: 6px; padding: 4px 0; cursor: pointer; font-size: 11px; color: {c.accent}; font-family: 'JetBrains Mono', monospace;",
                        onclick: move |_| on_open.call(mid.clone()),
                        span { class: "material-symbols-outlined", style: "font-size: 12px; color: {c.text_secondary};", "link" }
                        span { "[[{m.title}]]" }
                    }
                }
            })}
            if mentions.read().is_empty() && note_id.is_some() {
                p { style: "font-size: 10px; color: {c.text_muted}; font-family: Inter;", "No backlinks. Use [[Note Title]] to link." }
            }
        }
    }
}
