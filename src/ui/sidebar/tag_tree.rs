use crate::core::tag::TagService;
use crate::storage::models::Tag;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn TagTree(
    db: Option<std::sync::Arc<crate::storage::Database>>,
    selected_note_id: Option<String>,
    on_search_tag: EventHandler<String>,
) -> Element {
    let c = use_theme_colors();
    let mut roots = use_signal(Vec::<Tag>::new);

    if let Some(ref d) = db {
        if let Ok(r) = TagService::new(d).list_roots() {
            roots.set(r);
        }
    }

    rsx! {
        div { style: "padding: 8px 16px;",
            h4 { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 6px; font-family: 'JetBrains Mono', monospace;", "Tag Tree" }
            {roots.read().iter().map(|tag| {
                let tname = tag.name.clone();
                rsx! {
                    div { key: "{tag.id}", style: "padding: 3px 0; font-size: 11px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace; cursor: pointer;",
                        onclick: move |_| on_search_tag.call(tname.clone()),
                        div { style: "display: flex; align-items: center; gap: 4px;",
                            span { class: "material-symbols-outlined", style: "font-size: 14px; color: {c.accent};", "tag" }
                            span { "{tag.name}" }
                        }
                    }
                }
            })}
            if roots.read().is_empty() {
                p { style: "font-size: 10px; color: {c.text_muted}; padding-top: 4px;", "No tags yet. Add tags to your notes." }
            }
        }
    }
}
