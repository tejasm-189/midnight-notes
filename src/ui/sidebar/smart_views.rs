use crate::core::search::SearchService;
use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn SmartViewsList(
    db: Option<std::sync::Arc<crate::storage::Database>>,
    on_activate: EventHandler<String>,
) -> Element {
    let c = use_theme_colors();
    let mut views = use_signal(Vec::<(String, String)>::new);

    if let Some(ref d) = db {
        if let Ok(v) = SearchService::new(d).list_smart_views() {
            views.set(v);
        }
    }

    rsx! {
        div { style: "padding: 8px 16px;",
            h4 { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 6px; font-family: 'JetBrains Mono', monospace;", "Saved Views" }
            {views.read().iter().map(|(name, _query)| {
                let n = name.clone();
                rsx! {
                    div { key: "{name}", style: "display: flex; align-items: center; gap: 6px; padding: 4px 0; cursor: pointer; font-size: 11px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace;",
                        onclick: move |_| on_activate.call(n.clone()),
                        span { class: "material-symbols-outlined", style: "font-size: 14px; color: {c.accent};", "auto_awesome" }
                        span { "{name}" }
                    }
                }
            })}
            if views.read().is_empty() {
                p { style: "font-size: 10px; color: {c.text_muted}; padding-top: 4px;", "Save searches in Smart Views to see them here" }
            }
        }
    }
}
