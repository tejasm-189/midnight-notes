use crate::core::tag::TagService;
use crate::storage::models::Tag;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn TagTree(
    db: Option<std::sync::Arc<crate::storage::Database>>,
    selected_note_id: Option<String>,
) -> Element {
    let c = use_theme_colors();
    let mut roots = use_signal(Vec::<Tag>::new);
    let mut children = use_signal(Vec::<(String, Vec<Tag>)>::new);

    if let Some(ref d) = db {
        if let Ok(r) = TagService::new(d).list_roots() {
            roots.set(r);
        }
        let mut ch = Vec::new();
        for root in roots.read().iter() {
            if let Ok(c_tags) = TagService::new(d).get_children(&root.id) {
                ch.push((root.id.clone(), c_tags));
            }
        }
        children.set(ch);
    }

    rsx! {
        div { style: "padding: 8px 16px;",
            h4 { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 6px; font-family: 'JetBrains Mono', monospace;", "Tag Tree" }
            {roots.read().iter().map(|tag| {
                let has_children = children.read().iter().any(|(pid, _)| pid == &tag.id);
                rsx! {
                    div { key: "{tag.id}", style: "padding: 3px 0; font-size: 11px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace;",
                        div { style: "display: flex; align-items: center; gap: 4px;",
                            span { class: "material-symbols-outlined", style: "font-size: 14px; color: {c.accent};", if has_children { "folder" } else { "tag" } }
                            span { "{tag.name}" }
                        }
                        {children.read().iter().filter(|(pid, _)| pid == &tag.id).flat_map(|(_, kids)| {
                            kids.iter().map(|child| rsx! {
                                div { key: "{child.id}", style: "padding-left: 20px; padding-top: 2px; display: flex; align-items: center; gap: 4px; font-size: 10px; color: {c.text_muted};",
                                    span { class: "material-symbols-outlined", style: "font-size: 12px; color: {c.text_muted};", "subdirectory_arrow_right" }
                                    span { "{child.name}" }
                                }
                            })
                        })}
                    }
                }
            })}
            if roots.read().is_empty() {
                p { style: "font-size: 10px; color: {c.text_muted}; padding-top: 4px;", "No tags yet" }
            }
        }
    }
}
